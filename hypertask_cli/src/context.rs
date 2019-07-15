use chrono::prelude::*;
use hypertask_engine::prelude::*;
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::process::Command;
use std::{env, fs};

const ENV_VAR_SHELL: &str = "SHELL";
const ENV_VAR_DIR_NAME: &str = "HYPERTASK_DIR";
const ENV_VAR_AFTER_HOOK: &str = "HYPERTASK_AFTER";

pub struct CliContext {}

impl Context for CliContext {
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn get_input_tasks_iter(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Task, String>>>, String> {
        let hyper_cortex_dir = env::var(ENV_VAR_DIR_NAME)
            .map_err(|_| format!("environment variable {} is unset", ENV_VAR_DIR_NAME))?;

        let paths = fs::read_dir(&hyper_cortex_dir)
            .map_err(|_| format!("folder {} could not be found", hyper_cortex_dir.to_string()))?;

        Ok(Box::new(paths.map(|path| {
            path.map_err(|_| "Failed to open task".to_string())
                .and_then(|file_path| {
                    File::open(file_path.path())
                        .map_err(|_| format!("failed to open task `{:?}`", file_path))
                        .and_then(|file| {
                            serde_json::from_reader(BufReader::new(file))
                                .map_err(|_| format!("failed to parse task `{:?}`", file_path))?
                        })?
                })?
        })))
    }

    fn put_task(&self, task: &Task) -> Result<(), String> {
        let Id(task_id) = task.get_id();

        let hyper_cortex_dir = env::var(ENV_VAR_DIR_NAME)
            .map_err(|_| format!("environment variable {} is unset", ENV_VAR_DIR_NAME))?;

        let file_path = Path::new(&hyper_cortex_dir).join(task_id);

        let file = File::create(file_path).map_err(|_| "Unable to create file")?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, &task).map_err(|_| String::from("foo?"))?;

        if let (Ok(shell), Ok(after_cmd)) = (env::var(ENV_VAR_SHELL), env::var(ENV_VAR_AFTER_HOOK))
        {
            Command::new(shell)
                .arg("-c")
                .arg(after_cmd)
                .output()
                .map_err(|_| "Failed to execute command")?;
        }

        Ok(())
    }
}
