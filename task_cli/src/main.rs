extern crate hypercortex_engine;
extern crate task_cli;

use chrono::prelude::*;
use std::env;

fn get_now() -> DateTime<Utc> {
    Utc::now()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    task_cli::run_cli(&get_now, &args);
}
