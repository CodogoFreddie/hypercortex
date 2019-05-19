mod engine;
mod error;
mod id;
mod parse_args;
mod prop;
mod tag;
mod task;

use crate::error::{CortexError, CortexResult};
use crate::id::Id;
use crate::parse_args::parse_cli_args;
use crate::task::Task;
use chrono::prelude::*;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs};

type LoadedTask = Result<Task, String>;

fn get_tasks() -> impl Iterator<Item = LoadedTask> {
    let var_name = "HYPERCORTEX_DIR";

    let hyper_cortex_dir =
        env::var(var_name).expect(format!("environment variable {} is unset", var_name).as_str());

    let paths = fs::read_dir(&hyper_cortex_dir)
        .expect(format!("folder {} could not be found", hyper_cortex_dir.to_string()).as_str());

    paths.map(|path| {
        path.map_err(|e| "Failed to open task".to_string())
            .and_then(|file_path| {
                File::open(file_path.path())
                    .map_err(|e| format!("failed to open task `{:?}`", file_path))
                    .and_then(|file| {
                        serde_json::from_reader(BufReader::new(file))
                            .map_err(|e| format!("failed to parse task `{:?}`", file_path))
                    })
            })
    })
}

pub fn run_cli(get_now: &Fn() -> DateTime<Utc>, args: &Vec<String>) -> () {
    let tasks_iterator = get_tasks();

    let engine = parse_cli_args(args.iter().skip(1));

    //let tasks_to_display = engine.run(tasks_iterator);

    //println!("{}", tasks_to_display);
}
