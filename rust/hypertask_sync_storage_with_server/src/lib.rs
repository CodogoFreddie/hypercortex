#[macro_use]
extern crate log;
extern crate hypertask_engine;

use chrono::prelude::*;
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::ProvidesDataDir;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use time::Duration;
use wasm_bindgen::prelude::*;

type TaskHashes = HashMap<Rc<Id>, u64>;

pub trait ProvidesServerDetails: Sync + Send {
    fn get_server_url(&self) -> HyperTaskResult<&String>;
    fn get_server_secret_value(&self) -> HyperTaskResult<&String>;
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
    let uri = format!("{}/hashes", config.get_server_url()?);

    let mut res = surf::get(&uri)
        .set_header(
            "Authorization",
            format!("hypertask {}", config.get_server_secret_value()?),
        )
        .await?;

    let task_hashes_str = res.body_string().await?;

    let task_hashes = match serde_json::from_str(&task_hashes_str) {
        Ok(ok) => Ok(ok),
        Err(e) => {
            println!("{:?} `{}`", e, task_hashes_str);
            Err(e)
        }
    }?;

    Ok(task_hashes)
}

pub async fn get_remote_task_state<Config: ProvidesServerDetails>(
    config: &Config,
    id: &Id,
    client_task: &Option<Task>,
) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = format!("{}/task/{}", config.get_server_url()?, id);

    let server_task: Option<Task> = surf::post(uri)
        .set_header(
            "Authorization",
            format!("hypertask {}", config.get_server_secret_value()?),
        )
        .body_string(serde_json::to_string(&client_task).expect("could not serialise task"))
        .set_header("Content-Type", "application/json")
        .recv_json()
        .await?;

    Ok(server_task)
}

async fn sync_task_with_server<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
    id: &Rc<Id>,
) -> HyperTaskResult<()> {
    let local_task_state: Option<Task> = get_task(config, &*id)?;

    info!("got local task state `{:?}`", &local_task_state);

    let remote_task_state = get_remote_task_state(config, &**id, &local_task_state)
        .await
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
        })?;

    info!("got remote task state `{:?}`", &remote_task_state);

    let resolved_task = Task::resolve_task_conflict(local_task_state, remote_task_state)?;

    info!("resolved task conflict `{:?}`", &resolved_task);

    match resolved_task {
        Some(task) => {
            info!("save task");
            put_task(config, &task)?;
        }
        None => {
            info!("delete task");
            delete_task(config, id)?;
        }
    };

    Ok(())
}

pub async fn sync_all_tasks_async<Config: ProvidesDataDir + ProvidesServerDetails>(
    config: &Config,
) -> HyperTaskResult<()> {
    info!("running sync");

    let local_hashes = get_local_task_hash_map(config)?;

    info!("got local hashes");

    let remote_hashes = get_remote_task_hash_map(config).await.map_err(|e| {
        println!("{:?}", e);

        HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
            .msg("could not get remote hashes")
    })?;

    info!("got remote hashes");

    let mut ids: HashSet<Rc<Id>> = HashSet::new();

    for id in local_hashes.keys() {
        ids.insert(id.clone());
    }

    for id in remote_hashes.keys() {
        ids.insert(id.clone());
    }

    for id in &ids {
        if local_hashes.get(id) != remote_hashes.get(id) {
            info!(
                "found conflicting id hashes: `{}`, `{:?} != {:?}`",
                id,
                local_hashes.get(id),
                remote_hashes.get(id)
            );

            sync_task_with_server(config, id).await?;
        }
    }

    Ok(())
}
