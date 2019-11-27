extern crate hypertask_engine;

mod config;

use crate::config::SyncCliDaemonConfig;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-daemon.toml")?;
    let config_file_getter: ConfigFileGetter<SyncCliDaemonConfig> = config_file_opener.parse()?;

    Ok(())
}
