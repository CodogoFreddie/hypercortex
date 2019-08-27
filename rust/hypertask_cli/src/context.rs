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
struct ClientConfig {
    post_run_hook: Option<String>,
    pre_run_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct ServerConfig {
    port: Option<u32>,
    post_run_hook: Option<String>,
    pre_run_hook: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    data_dir: PathBuf,

    #[serde(default)]
    client: Option<ClientConfig>,

    #[serde(default)]
    server: Option<ServerConfig>,
}

impl Config {
    fn new() -> Self {
        let app_dirs = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        Self {
            data_dir: app_dirs.data_dir,
            ..Default::default()
        }
    }

    fn get_file_path() -> PathBuf {
        AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
            .unwrap()
            .config_dir
            .join("config.toml")
    }

    //creates the config file
    fn create_file() -> HyperTaskResult<()> {
        let default = Config::new();
        let stringified_default =
            toml::ser::to_string_pretty(&default).expect("can not format default config.toml");

        fs::write(Config::get_file_path(), stringified_default)
            .map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Create)
                    .msg("creating config file")
                    .from(e)
            })
            .map(|_| ())
    }

    //opens the config file
    //None if the file doesn't exist
    //Some(Err) if the file can't be parsed
    //Some(Ok(Config)) otherwise
    fn open_file() -> Option<HyperTaskResult<Self>> {
        fs::read_to_string(Config::get_file_path())
            .ok()
            .map(|stringified_config| {
                toml::de::from_str(&stringified_config).map_err(|e| {
                    HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Create)
                        .msg("could not parse current config.toml")
                        .from(e)
                })
            })
    }

    //opens the config file, creates it with defaults if it doesn't exist
    pub fn open_from_file() -> HyperTaskResult<Self> {
        Config::open_file().unwrap_or_else(|| {
            Config::create_file();

            Config::open_file().unwrap_or_else(|| {
                Err(
                    HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Create)
                        .msg("could not open created config.toml"),
                )
            })
        })
    }
}

#[derive(Debug, Default)]
pub struct CliContext {
    config: Config,
}

impl CliContext {
    pub fn new() -> HyperTaskResult<Self> {
        let config = Config::open_from_file()?;

        Ok(Self { config })
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

        let file_path = self.config.data_dir.join(task_id);

        let file = File::create(file_path).map_err(|_| "Unable to create file")?;
        let buf_writer = BufWriter::new(file);

        serde_json::to_writer_pretty(buf_writer, &task).map_err(|_| String::from("foo?"))?;

        //TODO fix this Option nesting
        if let (Ok(shell), Some(Some(after_cmd))) = (
            &env::var(ENV_VAR_SHELL),
            &self
                .config
                .client
                .as_ref()
                .map(|client| client.post_run_hook.as_ref()),
        ) {
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
        CliTaskIterator::new(&self.config.data_dir).unwrap()
    }
}
