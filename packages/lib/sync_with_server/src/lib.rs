#[macro_use]
extern crate log;
extern crate hypertask_engine;

use hypertask_engine::prelude::*;
use persisted_task_client::PersistedTaskClient;
use simple_persist_data::prelude::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

type TaskHashes<TaskPersister: PersistableMultiple<Task>> = HashMap<Rc<TaskPersister::ID>, u64>;

pub trait ProvidesServerDetails: Sync + Send {
    fn get_server_url(&self) -> HyperTaskResult<&String>;
    fn get_server_secret_value(&self) -> HyperTaskResult<&String>;
}

fn get_local_task_hash_map<TaskPersister: PersistableMultiple<Task>>() -> HyperTaskResult<TaskPersister> {
    let mut hm = HashMap::new();

    for id in PersistedTaskClient::get_all_ids()? {
        let PersistedTaskClient(task) = PersistedTaskClient::load_from_storage(&id)?;
        hm.insert(Rc::new(id), task.calculate_hash());
    }

    Ok(hm)
}

pub async fn get_remote_task_hash_map<Config: ProvidesServerDetails, TaskPersister: PersistableMultiple<Task>>(
    config: &Config,
) -> Result<TaskHashes<TaskPersister>, Box<dyn std::error::Error + Send + Sync + 'static>> {
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

pub async fn get_remote_task_state<
    Config: ProvidesServerDetails,
    TaskPersister: PersistableMultiple<Task>,
>(
    config: &Config,
    id: &Rc<TaskPersister::ID>,
    client_task: &Option<Task>,
) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    let uri = format!("{}/task/{}", config.get_server_url()?, id.as_ref().as_ref());

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

async fn sync_task_with_server<
    Config: ProvidesServerDetails,
    TaskPersister: PersistableMultiple<Task>,
>(
    config: &Config,
    id: &Rc<TaskPersister::ID>,
) -> HyperTaskResult<()> {
    let local_task_state: Option<Task> =
        TaskPersister::load_from_storage(id.as_ref())?.map(|task_persister| task_persister.into());

    info!("got local task state `{:?}`", &local_task_state);

    let remote_task_state = get_remote_task_state::<Config, TaskPersister>(config, id, &local_task_state)
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
            TaskPersister::from(task).save_to_storage()?;
        }
        None => {
            info!("delete task (not yet implemented)");
        }
    };

    Ok(())
}

pub async fn sync_all_tasks_async<
    Config: ProvidesServerDetails,
    TaskPersister: PersistableMultiple<Task>,
>(
    config: &Config,
) -> HyperTaskResult<()> {
    info!("running sync");

    let local_hashes = get_local_task_hash_map()?;

    info!("got local hashes");

    let remote_hashes: TaskHashes<TaskPersister> = get_remote_task_hash_map(config).await.map_err(|e| {
        println!("{:?}", e);

        HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
            .msg("could not get remote hashes")
    })?;

    info!("got remote hashes");

    let mut ids: HashSet<Rc<TaskPersister::ID>> = HashSet::new();

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

            sync_task_with_server::<Config, TaskPersister>(config, id).await?;
        }
    }

    Ok(())
}
