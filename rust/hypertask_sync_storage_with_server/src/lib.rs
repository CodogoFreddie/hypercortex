extern crate hypertask_engine;

use chrono::prelude::*;
use futures::executor::block_on;
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::ProvidesDataDir;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use time::Duration;

type TaskHashes = HashMap<Rc<Id>, u64>;

pub trait ProvidesServerDetails: Sync + Send {
    fn get_server_url(&self) -> &String;
    fn get_server_secret_value(&self) -> String;
}

pub fn get_local_task_hash_map<Config: ProvidesDataDir>(
    config: &Config,
) -> HyperTaskResult<TaskHashes> {
    let mut task_hashes = TaskHashes::new();

    let input_tasks: HashMap<Rc<Id>, Rc<Task>> = get_input_tasks(config)?;

    for (id, task) in input_tasks.iter() {
        task_hashes.insert(id.clone(), task.calculate_hash());
    }

    Ok(task_hashes)
}

pub async fn get_remote_task_hash_map<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
) -> Result<TaskHashes, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = format!("{}/hashes", config.get_server_url());

    let mut task_hashes_str_option = None;

    while task_hashes_str_option.is_none() {
        let mut res = surf::get(&uri)
            .set_header(
                "Authorization",
                format!("hypertask {}", config.get_server_secret_value()),
            )
            .await?;

        let headers = res.headers();
        let length: usize = headers
            .get("content-length")
            .expect("content-length not present")
            .parse()
            .expect("content-length not stringifiable");

        let task_hashes_str_possible = res.body_string().await?;

        // there's some fucking weird bug here that's causing task_hashes_str_possible to be cut short.
        // curling the url from the CLI works, and this code can correctly query other URLs,
        // but for some reason the combination of my server and my client is causing this
        // sporadic error.
        //
        // For now, we'll just keep re-try the query, as it's not super expensive, but #59 tracks
        // this issue
        if task_hashes_str_possible.len() == length {
            task_hashes_str_option = Some(task_hashes_str_possible);
        }
    }

    let task_hashes_str = task_hashes_str_option.unwrap();

    let task_hashes = match serde_json::from_str(&task_hashes_str) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            println!("{:?} `{}`", e, task_hashes_str);
            Err(e)
        }
    }?;

    println!("got remote hash map");

    Ok(task_hashes)
}

pub async fn get_remote_task_state<Config: ProvidesServerDetails>(
    config: &Config,
    id: &Id,
    task: &Option<Task>,
) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = format!("{}/task/{}", config.get_server_url(), id);
    let task: Option<Task> = surf::post(uri)
        .set_header(
            "Authorization",
            format!("hypertask {}", config.get_server_secret_value()),
        )
        .body_json(&task)?
        .recv_json()
        .await?;

    Ok(task)
}

async fn sync_task_with_server<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
    id: &Rc<Id>,
) -> HyperTaskResult<()> {
    let local_task_state: Option<Task> = get_task(config, &*id)?;
    let remote_task_state = get_remote_task_state(config, &**id, &local_task_state)
        .await
        .expect("could not get_remote_task_state");

    let resolved_task = Task::resolve_task_conflict(
        &(Utc::now() - Duration::days(30)),
        local_task_state,
        remote_task_state,
    )
    .expect("task ids did not match");

    match resolved_task {
        Some(task) => {
            put_task(config, &task)?;
        }
        None => {
            delete_task(config, id)?;
        }
    };

    Ok(())
}

pub async fn sync_all_tasks_tasks_async<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
) -> HyperTaskResult<()> {
    let local_hashes = get_local_task_hash_map(config)?;
    let remote_hashes = get_remote_task_hash_map(config).await.map_err(|e| {
        println!("{:?}", e);

        HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
            .msg("could not get remote hashes")
    })?;

    let mut ids: HashSet<Rc<Id>> = HashSet::new();

    for id in local_hashes.keys() {
        ids.insert(id.clone());
    }
    for id in remote_hashes.keys() {
        ids.insert(id.clone());
    }

    for id in &ids {
        if local_hashes.get(id) != remote_hashes.get(id) {
            sync_task_with_server(config, id).await?;
        }
    }

    Ok(())
}

pub fn sync_all_tasks<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
) -> HyperTaskResult<()> {
    let sync_future = sync_all_tasks_tasks_async(config);

    block_on(sync_future)
}
