#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate hypertask_engine;

mod parse_args;
mod render;

use crate::parse_args::parse_cli_args;
use crate::render::render_table;
use hypertask_cli_context::{run_string_as_shell_command, CliContext};
use hypertask_engine::prelude::*;
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::{env, fs};

const ENV_VAR_DIR_NAME: &str = "HYPERTASK_DIR";

pub fn run_cli(args: &[String]) -> Result<(), String> {
    let cli_context = CliContext::new_for_client()?;
    let after_hook = &cli_context.get_after_hook().clone();

    let command = parse_cli_args(args.iter().skip(1))?;
    let tasks_to_display = run(command, cli_context)?;

    render_table(&tasks_to_display);

    Ok(())
}
