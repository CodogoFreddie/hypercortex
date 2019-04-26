extern crate hypercortex_engine;

mod parse_args;

use parse_args::ParsedArgs;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let _input = ParsedArgs::new(args);
}
