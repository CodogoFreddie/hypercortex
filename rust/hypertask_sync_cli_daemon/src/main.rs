extern crate hypertask_sync_cli_daemon;

use hypertask_engine::prelude::*;

fn main() -> HyperTaskResult<()> {
    hypertask_sync_cli_daemon::start()
}
