extern crate hypertask_cli;

use hypertask_engine::prelude::*;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if let Err(e) = hypertask_cli::run_cli(&args) {
        print_error_chain(&e);
    }
}
