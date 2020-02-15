extern crate clap;
extern crate daemonize;
extern crate hypertask_sync_server;

use clap::Clap;
use daemonize::Daemonize;
use hypertask_engine::prelude::*;
use std::fs::File;
use std::path::PathBuf;

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
    port: u16,

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
}

fn main() -> HyperTaskResult<()> {
    let cli_args: CliArgs = CliArgs::parse();

    if cli_args.daemonize {
        let stdout = File::create("/tmp/hypertask-sync-server.out").unwrap();
        let stderr = File::create("/tmp/hypertask-sync-server.err").unwrap();

        let daemonize = Daemonize::new()
            .pid_file("/tmp/hypertask-sync-server.pid")
            .chown_pid_file(true)
            .stdout(stdout)
            .stderr(stderr)
            .exit_action(|| println!("daemonized server, logs can be found at `/tmp/hypertask-sync-server.out` & `/tmp/hypertask-sync-server.err`"))
            .privileged_action(|| "Executed before drop privileges");

        match daemonize.start() {
            Ok(_) => {
                println!("Success, daemonized");

                hypertask_sync_server::start()
            }
            Err(e) => {
                eprintln!("Error, {}", e);

                Err(
                    HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                        .msg("error forking sync server")
                        .from(e),
                )
            }
        }
    } else {
        hypertask_sync_server::start()
    }
}
