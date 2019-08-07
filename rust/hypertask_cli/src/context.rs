use chrono::prelude::*;
use hypertask_engine::prelude::*;
use platform_dirs::{AppDirs, AppUI};
use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

const ENV_VAR_SHELL: &str = "SHELL";

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CliContext {
    data_dir: PathBuf,

    #[serde(default)]
    run_after_hook_script: Option<String>,
}

impl CliContext {
    pub fn new() -> Result<Self, String> {
        let app_dirs = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        let config_file_path = app_dirs.config_dir.join("config.json");

        let file = File::open(&config_file_path).unwrap_or_else(|_| {
            println!(
                "no config file found at `{}`, one has been created with default values",
                &config_file_path.to_str().unwrap()
            );

            let default_context = CliContext {
                data_dir: app_dirs.data_dir,
                run_after_hook_script: None,
            };

            mkdirp::mkdirp(&app_dirs.config_dir).unwrap();
            let file = File::create(&config_file_path).unwrap();
            let buf_writer = BufWriter::new(&file);

            serde_json::to_writer_pretty(buf_writer, &default_context).unwrap();

            File::open(&config_file_path).unwrap()
        });

        serde_json::from_reader(BufReader::new(file))
            .and_then(|c: CliContext| {
                Ok(CliContext {
                    data_dir: PathBuf::from(
                        shellexpand::tilde(&c.data_dir.to_str().unwrap().to_string()).into_owned(),
                    ),
                    ..c
                })
            })
            .map_err(|e| {
                format!(
                    "could not open config file @ `{:?}` ({:?})",
                    config_file_path, e
                )
            })
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

        let file_path = self.data_dir.join(task_id);

        let file = File::create(file_path).map_err(|_| "Unable to create file")?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, &task).map_err(|_| String::from("foo?"))?;

        if let (Ok(shell), Some(after_cmd)) =
            (&env::var(ENV_VAR_SHELL), &self.run_after_hook_script)
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

pub struct CliTaskIterator {
    task_files_iterator: std::fs::ReadDir,
}

impl CliTaskIterator {
    pub fn new(data_dir: &PathBuf) -> Result<Self, String> {
        let task_files_iterator = fs::read_dir(&data_dir)
            .map_err(|_| format!("folder {:?} could not be found", data_dir.to_str()))?;

        Ok(Self {
            task_files_iterator,
        })
    }
}

impl Iterator for CliTaskIterator {
    type Item = Result<Task, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.task_files_iterator.next().map(|path| {
            path.map_err(|_| "Failed to open task".to_string())
                .and_then(|file_path| {
                    File::open(file_path.path())
                        .map_err(|_| format!("failed to open task `{:?}`", file_path))
                        .and_then(|file| {
                            serde_json::from_reader::<std::io::BufReader<std::fs::File>, Task>(
                                BufReader::new(file),
                            )
                            .map_err(|_| format!("failed to parse task @ `{:?}`", file_path))
                        })
                })
        })
    }
}

impl GetTaskIterator for CliContext {
    type TaskIterator = CliTaskIterator;

    fn get_task_iterator(&mut self) -> Self::TaskIterator {
        CliTaskIterator::new(&self.data_dir).unwrap()
    }
}
