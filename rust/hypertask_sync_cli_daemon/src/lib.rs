extern crate async_std;
extern crate hypertask_engine;

mod config;

use crate::config::SyncCliDaemonConfig;
use async_std::task;
use chrono::prelude::*;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use time::Duration;

type TaskHashes = HashMap<Rc<Id>, u64>;

pub fn get_local_task_hash_map(config: &SyncCliDaemonConfig) -> HyperTaskResult<TaskHashes> {
    let mut task_hashes = TaskHashes::new();

    let input_tasks: HashMap<Rc<Id>, Rc<Task>> = get_input_tasks(config)?;

    for (id, task) in input_tasks.iter() {
        task_hashes.insert(id.clone(), task.calculate_hash());
    }

    Ok(task_hashes)
}

pub fn get_remote_task_hash_map(
    config: &SyncCliDaemonConfig,
) -> Result<TaskHashes, Box<dyn std::error::Error + Send + Sync + 'static>> {
    task::block_on(async {
        let uri = format!("{}/hashes", config.server_url);
        let task_hashes: TaskHashes = surf::get(uri).recv_json().await?;
        Ok(task_hashes)
    })
}

pub fn get_remote_task_state(
    config: &SyncCliDaemonConfig,
    id: &Id,
    task: &Option<Task>,
) -> Result<Option<Task>, Box<dyn std::error::Error + Send + Sync + 'static>> {
    task::block_on(async {
        let uri = format!("{}/task/{}", config.server_url, id);
        let task: Option<Task> = surf::post(uri).body_json(&task)?.recv_json().await?;
        Ok(task)
    })
}

fn sync_task_with_server(config: &SyncCliDaemonConfig, id: &Rc<Id>) -> HyperTaskResult<()> {
    let local_task_state: Option<Task> = get_task(config, &*id)?;
    let remote_task_state = get_remote_task_state(config, &**id, &local_task_state).unwrap();

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

pub fn sync_all_tasks(config: &SyncCliDaemonConfig) -> HyperTaskResult<()> {
    let local_hashes = get_local_task_hash_map(config)?;
    let remote_hashes = get_remote_task_hash_map(config).map_err(|_| {
        HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
            .msg("could not get remote hashes")
    })?;

    dbg!((&local_hashes, &remote_hashes));

    let mut ids: HashSet<Rc<Id>> = HashSet::new();

    for id in local_hashes.keys() {
        ids.insert(id.clone());
    }
    for id in remote_hashes.keys() {
        ids.insert(id.clone());
    }

    for id in &ids {
        if local_hashes.get(id) != remote_hashes.get(id) {
            sync_task_with_server(config, id)?;
        }
    }

    dbg!(&ids);

    Ok(())
}

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-daemon.toml")?;
    let config_file_getter: ConfigFileGetter<SyncCliDaemonConfig> = config_file_opener.parse()?;

    sync_all_tasks(config_file_getter.get_config())?;
    Ok(())
}
