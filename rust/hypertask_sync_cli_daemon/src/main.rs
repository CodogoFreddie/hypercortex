extern crate daemonize;
extern crate hypertask_sync_cli_daemon;

use std::fs::File;

use daemonize::Daemonize;

use hypertask_engine::prelude::*;

fn main() -> HyperTaskResult<()> {
    let stdout = File::create("/tmp/hypertask-sync-cli-daemon.out").unwrap();
    let stderr = File::create("/tmp/hypertask-sync-cli-daemon.err").unwrap();

    let daemonize = Daemonize::new()
        .pid_file("/tmp/hypertask-sync-cli-daemon.pid")
        .chown_pid_file(true)
        .stdout(stdout)
        .stderr(stderr)
        .exit_action(|| println!("Executed before master process exits"))
        .privileged_action(|| "Executed before drop privileges");

    match daemonize.start() {
        Ok(_) => {
            println!("Success, daemonized");

            hypertask_sync_cli_daemon::start()
        }
        Err(e) => {
            eprintln!("Error, {}", e);

            Err(
                HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
                    .msg("error forking sync cli daemon")
                    .from(e),
            )
        }
    }
}
