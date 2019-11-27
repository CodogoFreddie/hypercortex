use hypertask_config_file_opener::ShellExpand;
use platform_dirs::{AppDirs, AppUI};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncCliDaemonConfig {
    pub data_dir: PathBuf,
    pub sync_secret_file: PathBuf,
    pub server_url: String,
}

impl Default for SyncCliDaemonConfig {
    fn default() -> Self {
        let AppDirs {
            data_dir,
            config_dir,
            ..
        } = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        Self {
            data_dir: data_dir,
            sync_secret_file: config_dir.join("daemon-sync-secret.txt"),
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
