use hypertask_engine::prelude::*;
use std::env;
use std::path::PathBuf;
use std::process::Command;

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
