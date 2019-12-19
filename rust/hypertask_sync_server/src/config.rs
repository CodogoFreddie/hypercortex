use hypertask_config_file_opener::ShellExpand;
use hypertask_task_io_operations::ProvidesDataDir;
use platform_dirs::{AppDirs, AppUI};
use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub const VALID_ID_CHARS: &str = "0123456789ABCDEFGHIJKLNMOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
pub const NUMBER_OF_CHARS_IN_FULL_ID: usize = 64;

fn generate_sync_secret() -> String {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncServerConfig {
    pub task_state_dir: PathBuf,
    pub sync_secret: String,
    pub hostname: String,
    pub port: u16,
}

impl ProvidesDataDir for SyncServerConfig {
    fn get_task_state_dir(&self) -> &PathBuf {
        &self.task_state_dir
    }
}

impl Default for SyncServerConfig {
    fn default() -> Self {
        let config_task_state_dir: PathBuf =
            AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
                .unwrap()
                .data_dir;

        Self {
            task_state_dir: config_task_state_dir,
            sync_secret: generate_sync_secret(),
            hostname: "localhost".to_owned(),
            port: 1234,
        }
    }
}

impl ShellExpand for SyncServerConfig {
    fn shell_expand(&mut self) {
        let task_state_dir_str: &str = self
            .task_state_dir
            .to_str()
            .expect("could not string from task_state_dir");

        let expanded_task_state_dir = shellexpand::tilde(task_state_dir_str);

        self.task_state_dir = PathBuf::from(expanded_task_state_dir.into_owned());
    }
}
