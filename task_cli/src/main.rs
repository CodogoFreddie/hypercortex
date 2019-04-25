extern crate engine;

mod parse_args;

use parse_args::ParsedArgs;
use std::env;

fn main() {
    let args: Vec<_> = env::args().collect();

    let input = ParsedArgs::new(args);
}
