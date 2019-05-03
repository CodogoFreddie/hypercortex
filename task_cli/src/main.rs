extern crate hypercortex_engine;
extern crate task_cli;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();
    task_cli::run(&args);
}
