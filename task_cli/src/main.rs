extern crate hypercortex_engine;

mod parse_args;

use chrono::prelude::*;
use parse_args::ParsedArgs;
use std::env;

fn get_now() -> DateTime<Utc> {
    Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)
}

fn main() {
    let args: Vec<_> = env::args().collect();

    let _input = ParsedArgs::new(&get_now, args);
}
