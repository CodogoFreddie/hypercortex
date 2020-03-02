#[macro_use]
extern crate log;
extern crate clap;
extern crate daemonize;
extern crate hypertask_engine;
extern crate notify;

mod cli_args;

use crate::clap::Clap;
use async_std::task;
use cli_args::CliArgs;
use daemonize::Daemonize;
use futures::try_join;
use hypertask_engine::prelude::*;
use std::fs::File;
use std::time::Duration;

use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;

async fn watch_for_changes(cli_args: &CliArgs) -> HyperTaskResult<()> {
    let debounce_seconds = cli_args.watch_debounce.unwrap_or(2);

    info!(
        "watching {:?} for changes (with a debounce of {} seconds)",
        &cli_args.data_dir, &debounce_seconds
    );

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(debounce_seconds)).unwrap();

    watcher
        .watch(&cli_args.data_dir, RecursiveMode::Recursive)
        .unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                info!("watched folder updated: {:?}", event);

                hypertask_sync_storage_with_server::sync_all_tasks_async(cli_args).await?;
            }
            Err(e) => error!("watch error: {:?}", e),
        }
    }
}

async fn run_at_interval(cli_args: &CliArgs, interval: u64) -> HyperTaskResult<()> {
    info!(
        "starting running with a rescan interval of {} seconds",
        &interval
    );

    loop {
        info!("syncing after interval of {} seconds", &interval);
        hypertask_sync_storage_with_server::sync_all_tasks_async(cli_args).await?;

        task::sleep(Duration::from_secs(interval)).await;
    }
}

async fn begin(cli_args: CliArgs) -> HyperTaskResult<()> {
    info!("running sync at least once");
    hypertask_sync_storage_with_server::sync_all_tasks_async(&cli_args).await?;

    match (&cli_args.rescan_refresh_rate, &cli_args.watch_for_changes) {
        (Some(interval), true) => {
            try_join!(
                watch_for_changes(&cli_args),
                run_at_interval(&cli_args, *interval)
            )?;
            Ok(())
        }
        (Some(interval), false) => run_at_interval(&cli_args, *interval).await,
        (None, true) => watch_for_changes(&cli_args).await,
        (None, false) => {
            info!("no need to run again");
            Ok(())
        }
    }
}

fn main() -> HyperTaskResult<()> {
    env_logger::init();

    info!("started hypertask syncing client");

    let cli_args: CliArgs = CliArgs::parse();

    if cli_args.daemonize {
        info!("attempting to daemonize");

        let mut daemonizer = Daemonize::new();

        daemonizer = if let Some(std_out_file_path) = &cli_args.std_out_file {
            let stdout = File::create(std_out_file_path).unwrap();
            daemonizer.stdout(stdout)
        } else {
            daemonizer
        };

        daemonizer = if let Some(std_err_file_path) = &cli_args.std_err_file {
            let stderr = File::create(std_err_file_path).unwrap();
            daemonizer.stderr(stderr)
        } else {
            daemonizer
        };

        daemonizer = if let Some(pid_file_path) = &cli_args.pid_file {
            daemonizer.pid_file(pid_file_path).chown_pid_file(true)
        } else {
            daemonizer
        };

        daemonizer = daemonizer.exit_action(|| println!("daemonized client"));

        match daemonizer.start() {
            Ok(_) => {
                println!("Success, daemonized");

                task::block_on(begin(cli_args))
            }
            Err(e) => {
                eprintln!("Error, {}", e);

                Err(
                    HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                        .msg("error forking sync client")
                        .from(e),
                )
            }
        }
    } else {
        task::block_on(begin(cli_args))
    }
}
