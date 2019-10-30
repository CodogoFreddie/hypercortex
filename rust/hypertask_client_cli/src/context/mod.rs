use chrono::prelude::*;
use hypertask_config_file_opener::{
    run_string_as_shell_command, ConfigFileGetter, ConfigFileOpener, ShellExpand,
};
use hypertask_engine::prelude::*;
use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct DataDirConfig(PathBuf);

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HooksConfig {
    pub after: Option<String>,
    pub on_edit: Option<String>,
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RenderConfig {
    score_precision: u32,
}

impl Default for RenderConfig {
    fn default() -> Self {
        Self { score_precision: 3 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ScoreCalculatorConfig {
    Single(String),
    Multiple(Vec<String>),
}

impl Default for ScoreCalculatorConfig {
    fn default() -> Self {
        ScoreCalculatorConfig::Single("now @ due : -".to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CliConfig {
    data_dir: DataDirConfig,
    pub hooks: Option<HooksConfig>,
    render: RenderConfig,
    score_calculator: ScoreCalculatorConfig,
}

impl ShellExpand for CliConfig {
    fn shell_expand(&mut self) {
        let data_dir_str: &str = self
            .data_dir
            .0
            .to_str()
            .expect("could not string from data_dir");

        let expanded_data_dir = shellexpand::tilde(data_dir_str);

        self.data_dir = DataDirConfig(PathBuf::from(expanded_data_dir.into_owned()));
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
                    .msg("could not open task path for reading")
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

pub struct CliContext {
    config_file_getter: ConfigFileGetter<CliConfig>,
}

impl CliContext {
    pub fn new() -> HyperTaskResult<CliContext> {
        let mut config_file_opener = ConfigFileOpener::new("client.toml")?;
        let config_file_getter = config_file_opener.parse()?;

        Ok(CliContext { config_file_getter })
    }
}

impl HyperTaskEngineContext<CliTaskIterator> for CliContext {
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn put_task(&mut self, task: &Task) -> HyperTaskResult<()> {
        let Id(task_id) = task.get_id();

        let file_path = self
            .config_file_getter
            .get_config()
            .data_dir
            .0
            .join(task_id);

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
        if let Some(HooksConfig {
            on_edit: Some(on_edit_cmd),
            ..
        }) = &self.config_file_getter.get_config().hooks
        {
            run_string_as_shell_command(&on_edit_cmd)?;
        }

        Ok(())
    }

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

    fn get_task_iterator(&self) -> HyperTaskResult<CliTaskIterator> {
        CliTaskIterator::new(&self.config_file_getter.get_config().data_dir.0).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                .msg("could not open tasks folder for reading")
                .from(e)
        })
    }

    fn finalize_mutations(&self) -> HyperTaskResult<()> {
        if let Some(hooks) = &self.config_file_getter.get_config().hooks {
            if let Some(after_cmd) = &hooks.after {
                match run_string_as_shell_command(after_cmd) {
                    Ok(output) => println!("{}", output),
                    Err(output) => println!("{}", output),
                }
            }
        };

        Ok(())
    }

    fn get_stack_machine(&self) -> HyperTaskResult<StackMachine> {
        let mut env = HashMap::new();

        let now = self.get_now();

        env.insert("day_of_week", f64::from(now.weekday().number_from_monday()));
        env.insert("hour", f64::from(now.hour()));
        env.insert("minute", f64::from(now.minute()));
        env.insert("month", f64::from(now.month()));
        env.insert("now", now.timestamp() as f64);

        let program = match &self.config_file_getter.get_config().score_calculator {
            ScoreCalculatorConfig::Single(s) => RPNSymbol::parse_program(s),
            ScoreCalculatorConfig::Multiple(ss) => RPNSymbol::parse_programs(&ss[..]),
        };

        Ok(StackMachine::new(program, env))
    }
}
