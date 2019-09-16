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

    fn get_file_path() -> PathBuf {
        AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
            .unwrap()
            .config_dir
            .join("config.toml")
    }

    //creates the config file
    fn create_file() -> HyperTaskResult<()> {
        let default = Self::new();
        let stringified_default =
            toml::ser::to_string_pretty(&default).expect("can not format default config.toml");

        fs::write(ConfigFile::get_file_path(), stringified_default)
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
    //Some(Ok(ConfigFile)) otherwise
    fn open_file() -> Option<HyperTaskResult<Self>> {
        fs::read_to_string(ConfigFile::get_file_path())
            .ok()
            .map(|stringified_config| {
                toml::de::from_str(&stringified_config).map_err(|e| {
                    HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Read)
                        .msg("could not parse current config.toml")
                        .from(e)
                })
            })
    }

    //opens the config file, creates it with defaults if it doesn't exist
    pub fn open_from_file() -> HyperTaskResult<Self> {
        ConfigFile::open_file().unwrap_or_else(|| {
            ConfigFile::create_file();

            ConfigFile::open_file().unwrap_or_else(|| {
                Err(
                    HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Create)
                        .msg("could not open created config.toml"),
                )
            })
        })
    }

    pub fn get_data_dir(&self) -> HyperTaskResult<PathBuf> {
        let data_dir_path_string = self.data_dir.to_str().ok_or(
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                .msg("can't read data_dir"),
        )?;

        shellexpand::full(data_dir_path_string)
            .map(|expanded_data_dir| {
                let mut path = PathBuf::new();
                path.push(expanded_data_dir.into_owned());
                path
            })
            .map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                    .with_msg(|| format!("can't expand data_dir `{}`", data_dir_path_string))
                    .from(e)
            })
    }
}
