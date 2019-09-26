extern crate hypertask_engine;
extern crate platform_dirs;

use hypertask_engine::prelude::*;
use platform_dirs::{AppDirs, AppUI};
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub trait ShellExpand {
    fn shell_expand(&mut self) -> ();
}
pub struct ConfigFileOpener<'a, T: ShellExpand + Default + Deserialize<'a> + Serialize> {
    config: Option<T>,
    config_source: String,
    phantom: PhantomData<&'a T>,
}

pub struct ConfigFileGetter<T> {
    config: T,
}

impl<T> ConfigFileGetter<T> {
    pub fn get_config(&self) -> &T {
        &self.config
    }
}

impl<'a, T: 'a + ShellExpand + Clone + Default + Deserialize<'a> + Serialize>
    ConfigFileOpener<'a, T>
{
    fn create_file(config_file_path: &str) -> HyperTaskResult<()> {
        let default = T::default();

        let stringified_default = toml::ser::to_string_pretty(&default).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Create)
                .from(e)
                .with_msg(|| format!("could not create {}", config_file_path))
        })?;

        fs::write(config_file_path, stringified_default).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Write)
                .from(e)
                .with_msg(|| format!("could not write {}", config_file_path))
        })?;

        Ok(())
    }

    fn unwrap_stringified_file_creating_default(
        config_file_path: &str,
        stringified_file: std::io::Result<String>,
    ) -> HyperTaskResult<String> {
        match stringified_file {
            Ok(s) => Ok(s),
            Err(_) => {
                Self::create_file(config_file_path);

                fs::read_to_string(config_file_path).map_err(|e| {
                    HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Read)
                        .from(e)
                        .with_msg(|| format!("could not read {}", config_file_path))
                })
            }
        }
    }

    pub fn new(config_file_name: &str) -> HyperTaskResult<Self> {
        let mut config_file_path: PathBuf = AppDirs::new(Some("hypertask-cli"), AppUI::CommandLine)
            .unwrap()
            .config_dir;

        config_file_path.push(config_file_name);

        let config_source = Self::unwrap_stringified_file_creating_default(
            &config_file_path
                .to_str()
                .expect("could not stringify client config file path"),
            fs::read_to_string(&config_file_path),
        )?;

        Ok(Self {
            config: None,
            config_source,
            phantom: PhantomData,
        })
    }

    pub fn parse(&'a mut self) -> HyperTaskResult<ConfigFileGetter<T>> {
        let mut config: T = toml::de::from_str(&self.config_source).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Parse)
                .from(e)
                .with_msg(|| format!("could not parse config"))
        })?;

        config.shell_expand();

        Ok(ConfigFileGetter { config })
    }
}

const ENV_VAR_SHELL: &str = "SHELL";

pub fn run_string_as_shell_command(cmd: &str) -> HyperTaskResult<String> {
    if let Ok(shell) = env::var(ENV_VAR_SHELL) {
        Command::new(shell)
            .arg("-c")
            .arg(cmd)
            .output()
            .map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Run)
                    .with_msg(|| format!("could not run the post write shell command `{}`", cmd))
                    .from(e)
            })
            .and_then(|output| {
                let stdout = std::str::from_utf8(&output.stdout)
                    .map(|s| s.to_owned())
                    .map_err(|e| {
                        HyperTaskError::new(
                            HyperTaskErrorDomain::Context,
                            HyperTaskErrorAction::Run,
                        )
                        .with_msg(|| {
                            format!(
                                "could not return the stdout of the post write shell command `{}`",
                                cmd
                            )
                        })
                        .from(e)
                    })?;

                let stderr = std::str::from_utf8(&output.stderr)
                    .map(|s| s.to_owned())
                    .map_err(|e| {
                        HyperTaskError::new(
                            HyperTaskErrorDomain::Context,
                            HyperTaskErrorAction::Run,
                        )
                        .with_msg(|| {
                            format!(
                                "could not return the stderr of the post write shell command `{}`",
                                cmd
                            )
                        })
                        .from(e)
                    })?;

                Ok(format!("{}{}", stdout, stderr))
            })
    } else {
        Err(
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Run)
                .msg("could not get the current shell"),
        )
    }
}
