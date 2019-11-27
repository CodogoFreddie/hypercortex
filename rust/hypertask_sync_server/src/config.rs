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
    pub data_dir: PathBuf,
    pub sync_secret: String,
    pub hostname: String,
    pub port: u16,
}

impl ProvidesDataDir for SyncServerConfig {
    fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

impl Default for SyncServerConfig {
    fn default() -> Self {
        let config_data_dir: PathBuf = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
            .unwrap()
            .data_dir;

        Self {
            data_dir: config_data_dir,
            sync_secret: generate_sync_secret(),
            hostname: "localhost".to_owned(),
            port: 1234,
        }
    }
}

impl ShellExpand for SyncServerConfig {
    fn shell_expand(&mut self) {
        let data_dir_str: &str = self
            .data_dir
            .to_str()
            .expect("could not string from data_dir");

        let expanded_data_dir = shellexpand::tilde(data_dir_str);

        self.data_dir = PathBuf::from(expanded_data_dir.into_owned());
    }
}
