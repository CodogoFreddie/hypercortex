use chrono::prelude::*;
use hypertask_engine::prelude::*;
use platform_dirs::{AppDirs, AppUI};
use rand::seq::IteratorRandom;
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

const ENV_VAR_SHELL: &str = "SHELL";
const ENV_VAR_DIR_NAME: &str = "HYPERTASK_DIR";
const ENV_VAR_AFTER_HOOK: &str = "HYPERTASK_AFTER";

pub struct CliContext {
    data_dir: PathBuf,
}

impl CliContext {
    pub fn new() -> Self {
        let app_dirs = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        dbg!(&app_dirs.config_dir.join("config.toml"));

        Self {
            data_dir: app_dirs.data_dir,
        }
    }
}

impl GetNow for CliContext {
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl GenerateId for CliContext {
    fn generate_id(&mut self) -> String {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = VALID_ID_CHARS
                .chars()
                .choose(&mut rand::thread_rng())
                .expect("Couldn't get random char");

            result.push(random);
        }

        result
    }
}

impl PutTask for CliContext {
    fn put_task(&mut self, task: &Task) -> Result<(), String> {
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
