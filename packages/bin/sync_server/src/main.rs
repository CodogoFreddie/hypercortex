#[macro_use]
extern crate log;
extern crate clap;
extern crate daemonize;
extern crate hypertask_engine;

mod cli_args;
mod server;
mod sync_secret;

use crate::clap::Clap;
use async_std::task;
use cli_args::CliArgs;
use daemonize::Daemonize;
use hypertask_engine::prelude::*;
use std::fs::File;

fn main() -> HyperTaskResult<()> {
    env_logger::init();

    info!("started hypertask syncing server");

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

        daemonizer = daemonizer.exit_action(|| info!("daemonized server"));

        match daemonizer.start() {
            Ok(_) => {
                info!("Success, daemonized");

                task::block_on(server::start(cli_args))
            }
            Err(e) => {
                error!("Error, {}", e);

                Err(
                    HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                        .msg("error forking sync server")
                        .from(e),
                )
            }
        }
    } else {
        task::block_on(server::start(cli_args))
    }
}
