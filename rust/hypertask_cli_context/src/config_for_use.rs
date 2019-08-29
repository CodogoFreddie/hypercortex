use crate::config_file::ConfigFile;
use hypertask_engine::prelude::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

const ENV_VAR_SHELL: &str = "SHELL";

pub fn run_string_as_shell_command(cmd: &String) -> HyperTaskResult<String> {
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
                std::str::from_utf8(&output.stdout)
                    .map(|s| s.to_owned())
                    .map_err(|e| {
                        HyperTaskError::new(
                            HyperTaskErrorDomain::Context,
                            HyperTaskErrorAction::Run,
                        )
                        .with_msg(|| {
                            format!(
                                "could not return the output of the post write shell command `{}`",
                                cmd
                            )
                        })
                        .from(e)
                    })
            })
    } else {
        Err(
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Run)
                .msg("could not get the current shell"),
        )
    }
}

#[derive(Debug, Default)]
pub struct ConfigForUse {
    pub data_dir: PathBuf,
    pub hook_after: Option<String>,
    pub hook_before: Option<String>,
    pub server_port: Option<u16>,
    pub server_address: Option<String>,
}

impl ConfigForUse {
    pub fn new_for_client() -> HyperTaskResult<Self> {
        let config = ConfigFile::open_from_file()?;
        let data_dir = config.get_data_dir()?;

        let mut hook_after = None;
        let mut hook_before = None;

        if let Some(hooks) = config.client.and_then(|client| client.hooks) {
            hook_after = hooks.after;
            hook_before = hooks.before;
        };

        Ok(Self {
            data_dir,
            server_port: None,
            server_address: None,
            hook_after,
            hook_before,
        })
    }

    pub fn new_for_server() -> HyperTaskResult<Self> {
        let config = ConfigFile::open_from_file()?;
        let data_dir = config.get_data_dir()?;

        let server_config = config.server.ok_or(
            HyperTaskError::new(HyperTaskErrorDomain::Config, HyperTaskErrorAction::Read)
                .msg("missing [server] section from config.toml"),
        )?;

        Ok(Self {
            data_dir,
            server_port: Some(server_config.port),
            server_address: server_config.address,
            hook_after: server_config.hooks.after,
            hook_before: server_config.hooks.before,
        })
    }
}
