extern crate async_std;
extern crate crossbeam_channel;
extern crate hypertask_engine;
extern crate notify;

mod config;

use crate::config::SyncCliDaemonConfig;
use async_std::task;
use chrono::prelude::*;
use clap::Clap;
use crossbeam_channel::unbounded;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use time::Duration;

/// Syncing server to replicate hypertask tasks with clients over HTTP
#[derive(Clap)]
struct CliArgs {
    /// Directory containing tasks
    #[clap(short = "d", long = "data")]
    task_state_dir: PathBuf,

    /// Should the server daemonise
    #[clap(long = "daemonize")]
    daemonize: bool,

    /// The hostname that the server will listen under
    #[clap(short = "h", long = "hostname")]
    hostname: Option<String>,

    /// The port that the server will listen with
    #[clap(short = "p", long = "port")]
    port: Option<u16>,

    /// The authorisation secret that must be passed by the client.
    /// The server will generate one if you do not specify
    #[clap(short = "s", long = "secret")]
    sync_secret: Option<String>,

    /// File to divert stdout to
    #[clap(short = "o", long = "out-file")]
    std_out_file: Option<PathBuf>,

    /// File to divert stderr to
    #[clap(short = "e", long = "err-file")]
    std_err_file: Option<PathBuf>,

    /// File to store PID in
    #[clap(long = "pid")]
    pid_file: Option<PathBuf>,

    /// How frequently in seconds to perform a manual rescan
    #[clap(short = "r" long = "rescan-rate")]
    rescan_rate: Option<u64>,
}

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

        let mut res = surf::get(&uri)
            .set_header(
                "Authorization",
                format!("hypertask {}", config.sync_secret.get_secret_value()),
            )
            .await?;

        let headers = res.headers();
        let length: usize = headers
            .get("content-length")
            .expect("content-length not present")
            .parse()
            .expect("content-length not stringifiable");

        let task_hashes_str = res.body_string().await?;

        // there's some fucking weird bug here that's causing task_hashes_str to be cut short.
        // curling the url from the CLI works, and this code can correctly query other URLs,
        // but for some reason the combination of my server and my client is causing this
        // sporadic error.
        //
        // For now, we'll just re-try the query, as it's not super expensive, but #59 tracks this
        // issue
        if task_hashes_str.len() != length {
            return get_remote_task_hash_map(config);
        }

        let task_hashes = match serde_json::from_str(&task_hashes_str) {
            Ok(ok) => Ok(ok),
            Err(e) => {
                println!("{:?} `{}`", e, task_hashes_str);
                Err(e)
            }
        }?;

        println!("got remote hash map");

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
        let task: Option<Task> = surf::post(uri)
            .set_header(
                "Authorization",
                format!("hypertask {}", config.sync_secret.get_secret_value()),
            )
            .body_json(&task)?
            .recv_json()
            .await?;
        Ok(task)
    })
}

fn sync_task_with_server(config: &SyncCliDaemonConfig, id: &Rc<Id>) -> HyperTaskResult<()> {
    let local_task_state: Option<Task> = get_task(config, &*id)?;
    let remote_task_state = get_remote_task_state(config, &**id, &local_task_state)
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

pub fn sync_all_tasks(config: &SyncCliDaemonConfig) -> HyperTaskResult<()> {
    let local_hashes = get_local_task_hash_map(config)?;
    let remote_hashes = get_remote_task_hash_map(config).map_err(|e| {
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
            sync_task_with_server(config, id)?;
        }
    }

    Ok(())
}

pub fn start() -> HyperTaskResult<()> {
    let cli_options = CliOptions::parse();

    //let mut config_file_opener = ConfigFileOpener::new("sync-daemon.toml")?;
    //let config_file_getter: ConfigFileGetter<SyncCliDaemonConfig> = config_file_opener.parse()?;

    //sync_all_tasks(config_file_getter.get_config())?;

    //let (tx, rx) = unbounded();

    //let mut watcher: RecommendedWatcher = Watcher::new(tx, std::time::Duration::from_secs(5))
    //.map_err(|e| {
    //HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
    //.msg("could not create task_state_dir watcher")
    //.from(e)
    //})?;

    //watcher
    //.watch(
    //config_file_getter.get_config().task_state_dir.clone(),
    //RecursiveMode::Recursive,
    //)
    //.map_err(|e| {
    //HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
    //.msg("error watching task_state_dir")
    //.from(e)
    //})?;

    //loop {
    //match rx.recv() {
    //Ok(_) => {
    //match sync_all_tasks(config_file_getter.get_config()) {
    //Ok(_) => println!("synced"),
    //Err(e) => println!("sync error: {:?}", e),
    //};
    //}
    //Err(err) => println!("watch error: {:?}", err),
    //};
    //}
}

fn main() {}
