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
use serde_json;
use std::fs::File;
use std::io::BufReader;
use std::{env, fs};

const ENV_VAR_DIR_NAME: &str = "HYPERTASK_DIR";

pub fn run_cli(args: &[String]) -> Result<(), String> {
    let cli_context = CliContext::new()?;

    let hyper_cortex_dir = env::var(ENV_VAR_DIR_NAME)
        .map_err(|_| format!("environment variable {} is unset", ENV_VAR_DIR_NAME))?;

    let paths = fs::read_dir(&hyper_cortex_dir)
        .map_err(|_| format!("folder {} could not be found", hyper_cortex_dir.to_string()))?;

    let command = parse_cli_args(args.iter().skip(1))?;
    let tasks_to_display = run(command, cli_context)?;

    render_table(&tasks_to_display);

    Ok(())
}
