use hypertask_engine::prelude::*;
use platform_dirs::{AppDirs, AppUI};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct HooksConfig {
    pub after: Option<String>,
    pub on_edit: Option<String>,
    pub before: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ClientConfig {
    pub hooks: Option<HooksConfig>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ServerConfig {
    pub hooks: HooksConfig,
    pub port: u16,

    #[serde(default)]
    pub address: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ConfigFile {
    data_dir: PathBuf,

    #[serde(default)]
    pub client: Option<ClientConfig>,

    #[serde(default)]
    pub server: Option<ServerConfig>,
}

impl ConfigFile {
    fn new() -> Self {
        let app_dirs = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine).unwrap();

        Self {
            data_dir: app_dirs.data_dir,
            ..Default::default()
        }
    }

    pub fn get_server_config(&self) -> &Option<ServerConfig> {
        &self.server
    }
}
