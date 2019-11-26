extern crate hypertask_engine;

mod config;

use crate::config::SyncServerConfig;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-server.toml")?;
    let config_file_getter: ConfigFileGetter<SyncServerConfig> = config_file_opener.parse()?;
    let sync_server_config: &SyncServerConfig = config_file_getter.get_config();

    dbg!(sync_server_config);

    Ok(())
}
