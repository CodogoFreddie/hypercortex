extern crate hypertask_sync_server;

use hypertask_engine::prelude::*;

fn main() -> HyperTaskResult<()> {
    hypertask_sync_server::start()
}
