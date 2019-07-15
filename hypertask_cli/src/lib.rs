#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate hypertask_engine;

mod context;
mod parse_args;
mod render;

use crate::context::CliContext;
use crate::parse_args::parse_cli_args;
use crate::render::render_table;
use hypertask_engine::prelude::*;

pub fn run_cli(args: &[String]) -> Result<(), String> {
    let cli_context = CliContext {};

    let command = parse_cli_args(args.iter().skip(1))?;
    let tasks_to_display = run(command, &cli_context)?;

    render_table(&tasks_to_display);

    Ok(())
}
