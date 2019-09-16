mod config_file;
mod config_for_use;

pub use crate::config_for_use::run_string_as_shell_command;
use crate::config_for_use::ConfigForUse;
use chrono::prelude::*;
use hypertask_engine::prelude::*;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde_json;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

#[derive(Debug, Default)]
pub struct CliContext {
    config: ConfigForUse,
}

impl CliContext {
    pub fn new_for_client() -> HyperTaskResult<Self> {
        Ok(Self {
            config: ConfigForUse::new_for_client()?,
        })
    }

    pub fn new_for_server() -> HyperTaskResult<Self> {
        Ok(Self {
            config: ConfigForUse::new_for_server()?,
        })
    }

    pub fn get_after_hook(&self) -> &Option<String> {
        &self.config.hook_after
    }

    pub fn get_data_dir(&self) -> &PathBuf {
        &self.config.data_dir
    }

    pub fn get_server_port(&self) -> &Option<u16> {
        &self.config.server_port
    }

    pub fn get_server_address(&self) -> &Option<String> {
        &self.config.server_address
    }
}

impl GetNow for CliContext {
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl PutTask for CliContext {
    fn put_task(&mut self, task: &Task) -> HyperTaskResult<()> {
        let Id(task_id) = task.get_id();

        let file_path = self.get_data_dir().join(task_id);

        let file = File::create(file_path).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
                .with_msg(|| {
                    format!(
                        "could not create file handle for task with id `{}`",
                        task_id
                    )
                })
                .from(e)
        })?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, &task).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
                .with_msg(|| format!("could not serialize task with id `{}`", task_id))
                .from(e)
        })?;

        //TODO fix this Option nesting
        if let Some(on_edit_cmd) = &self.config.hook_on_edit {
            let output = run_string_as_shell_command(on_edit_cmd)?;
        }

        Ok(())
    }
}

impl GenerateId for CliContext {
    fn generate_id(&mut self) -> String {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = VALID_ID_CHARS
                .chars()
                .choose(&mut thread_rng())
                .expect("Couldn't get random char");

            result.push(random);
        }

        result
    }
}

pub struct CliTaskIterator {
    task_files_iterator: std::fs::ReadDir,
}

impl CliTaskIterator {
    pub fn new(data_dir: &PathBuf) -> HyperTaskResult<Self> {
        let task_files_iterator = fs::read_dir(&data_dir).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                .with_msg(|| {
                    format!(
                        "folder `{:}` could not be found",
                        &data_dir.to_str().unwrap_or("")
                    )
                })
                .from(e)
        })?;

        Ok(Self {
            task_files_iterator,
        })
    }
}

impl Iterator for CliTaskIterator {
    type Item = HyperTaskResult<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        self.task_files_iterator.next().map(|path| {
            path.map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    .msg("could not open task folder for reading")
                    .from(e)
            })
            .and_then(|file_path| {
                File::open(file_path.path())
                    .map_err(|e| {
                        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                            .with_msg(|| format!("failed to open task `{:?}`", file_path))
                            .from(e)
                    })
                    .and_then(|file| {
                        serde_json::from_reader::<std::io::BufReader<std::fs::File>, Task>(
                            BufReader::new(file),
                        )
                        .map_err(|e| {
                            HyperTaskError::new(
                                HyperTaskErrorDomain::Task,
                                HyperTaskErrorAction::Read,
                            )
                            .with_msg(|| format!("failed to parse task @ `{:?}`", file_path))
                            .from(e)
                        })
                    })
            })
        })
    }
}

impl GetTaskIterator for CliContext {
    type TaskIterator = CliTaskIterator;

    fn get_task_iterator(&self) -> HyperTaskResult<Self::TaskIterator> {
        CliTaskIterator::new(self.get_data_dir()).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                .msg("could not open tasks folder for reading")
                .from(e)
        })
    }
}

impl FinalizeMutations for CliContext {
    fn finalize_mutations(&self) -> HyperTaskResult<()> {
        Ok(if let Some(after_cmd) = self.config.hook_after.clone() {
            match run_string_as_shell_command(&after_cmd) {
                Ok(output) => println!("{}", output),
                Err(output) => println!("{}", output),
            }
        })
    }
}
