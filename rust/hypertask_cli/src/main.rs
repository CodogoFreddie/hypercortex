extern crate hypertask_cli;

use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    if let Err(s) = hypertask_cli::run_cli(&args) {
        println!("{}", s)
    }
}
