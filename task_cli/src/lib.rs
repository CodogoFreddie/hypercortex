extern crate hypercortex_engine;

mod parse_args;

use chrono::prelude::*;
use hypercortex_engine::GetNow;
use parse_args::ParsedArgs;
use std::env;

pub fn run(get_now: &GetNow) -> () {
    let args: Vec<_> = env::args().collect();
    let _input = ParsedArgs::new(get_now, args);
}
