extern crate hypercortex_engine;
extern crate task_cli;

use chrono::prelude::*;

fn get_now() -> DateTime<Utc> {
    Utc::now()
}

fn main() {
    task_cli::run(&get_now);
}
