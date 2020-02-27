#[macro_use]
extern crate log;
extern crate clap;
extern crate daemonize;
extern crate hypertask_engine;

mod cli_args;

use crate::clap::Clap;
use async_std::task;
use cli_args::CliArgs;
use daemonize::Daemonize;
use futures::try_join;
use hypertask_engine::prelude::*;
use std::fs::File;

async fn watch_for_changes(cli_args: &CliArgs) -> HyperTaskResult<()> {
    unimplemented!();
    Ok(())
}

async fn run_at_interval(cli_args: &CliArgs) -> HyperTaskResult<()> {
    unimplemented!();
    Ok(())
}

async fn begin(cli_args: CliArgs) -> HyperTaskResult<()> {
    match (&cli_args.rescan_refresh_rate, &cli_args.watch_for_changes) {
        (Some(_), true) => {
            try_join!(watch_for_changes(&cli_args), run_at_interval(&cli_args));
            Ok(())
        }
        (Some(_), false) => watch_for_changes(&cli_args).await,
        (None, true) => run_at_interval(&cli_args).await,
        (None, false) => hypertask_sync_storage_with_server::sync_all_tasks_async(&cli_args).await,
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
