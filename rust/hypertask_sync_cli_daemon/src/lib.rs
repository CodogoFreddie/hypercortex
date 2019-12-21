extern crate async_std;
extern crate crossbeam_channel;

extern crate hypertask_sync_storage_with_server;

mod config;

use crate::config::SyncCliDaemonConfig;
use crossbeam_channel::unbounded;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use notify::{RecommendedWatcher, RecursiveMode, Watcher};

use hypertask_sync_storage_with_server::sync_all_tasks;

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-daemon.toml")?;
    let config_file_getter: ConfigFileGetter<SyncCliDaemonConfig> = config_file_opener.parse()?;

    sync_all_tasks(config_file_getter.get_config())?;

    let (tx, rx) = unbounded();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(5))
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                .msg("could not create task_state_dir watcher")
                .from(e)
        })?;

    watcher
        .watch(
            config_file_getter.get_config().task_state_dir.clone(),
            RecursiveMode::Recursive,
        )
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                .msg("error watching task_state_dir")
                .from(e)
        })?;

    loop {
        match rx.recv() {
            Ok(_) => {
                match sync_all_tasks(config_file_getter.get_config()) {
                    Ok(_) => println!("synced"),
                    Err(e) => println!("sync error: {:?}", e),
                };
            }
            Err(err) => println!("watch error: {:?}", err),
        };
    }
}
