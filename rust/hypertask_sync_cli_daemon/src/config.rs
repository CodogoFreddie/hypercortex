use hypertask_config_file_opener::ShellExpand;
use hypertask_task_io_operations::ProvidesDataDir;
use platform_dirs::{AppDirs, AppUI};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum SyncSecretSource {
    EnvVar { var_name: String },
    FilePath { path: PathBuf },
}

impl SyncSecretSource {
    pub fn get_secret_value(&self) -> String {
        match self {
            SyncSecretSource::EnvVar { var_name } => env::var(var_name).unwrap(),
            SyncSecretSource::FilePath { path } => {
                fs::read_to_string(path).expect("Unable to read file")
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncCliDaemonConfig {
    pub data_dir: PathBuf,
    pub sync_secret: SyncSecretSource,
    pub server_url: String,
}

impl ProvidesDataDir for SyncCliDaemonConfig {
    fn get_data_dir(&self) -> &PathBuf {
        &self.data_dir
    }
}

impl Default for SyncCliDaemonConfig {
    fn default() -> Self {
        let AppDirs { data_dir, .. } =
            AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        Self {
            data_dir,
            sync_secret: SyncSecretSource::EnvVar {
                var_name: "HYPERTASK_DAEMON_SYNC_SECRET".to_owned(),
            },
            server_url: "https://hypertask-sync-server.horse:1234".to_owned(),
        }
    }
}

impl ShellExpand for SyncCliDaemonConfig {
    fn shell_expand(&mut self) {
        let data_dir_str: &str = self
            .data_dir
            .to_str()
            .expect("could not string from data_dir");

        let expanded_data_dir = shellexpand::tilde(data_dir_str);

        self.data_dir = PathBuf::from(expanded_data_dir.into_owned());
    }
}
