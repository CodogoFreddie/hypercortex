use chrono::prelude::*;
use hypercortex_engine::prelude::*;
use serde::Deserialize;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::{env, fs};

//mod parse_args;

type GenericError = Box<Error>;
type ParsedTaskResult = Result<Task, GenericError>;
type ParsedTaskIterator = Box<Iterator<Item = ParsedTaskResult>>;

fn get_tasks() -> Result<ParsedTaskIterator, GenericError> {
    let key = "HYPERCORTEX_DIR";

    let hyper_cortex_dir = match env::var(key) {
        Ok(x) => x,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    let paths = match fs::read_dir(hyper_cortex_dir) {
        Ok(x) => x,
        Err(e) => {
            return Err(Box::new(e));
        }
    };

    Ok(Box::new(paths.map(|path| {
        let file_path = path?;
        let file = File::open(file_path.path())?;

        let reader = BufReader::new(file);

        let task: Task = serde_json::from_reader(reader)?;

        Ok(task)
    })))
}

pub fn run_cli(get_now: &Fn() -> DateTime<Utc>, _args: &Vec<String>) -> () {
    let tasks_iterator = match get_tasks() {
        Err(fs_err) => {
            println!("Error opening HyperCortex: `{}`", fs_err);
            return ();
        }
        Ok(x) => x,
    };

    let persister = |task: Task| -> CortexResult<Task> { Ok(task) };

    let tasks_to_output = run(persister, Some(vec![]), Some(vec![]), tasks_iterator);
}
