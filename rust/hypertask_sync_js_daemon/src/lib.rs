#[macro_use]
extern crate lazy_static;

use hypertask_sync_storage_with_server::ProvidesServerDetails;
use hypertask_task_io_operations::ProvidesDataDir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref DEFAULT_PATH_BUF: PathBuf = { PathBuf::default() };
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncCliDaemonConfig {
    pub sync_secret: String,
    pub server_url: String,
}

impl ProvidesDataDir for SyncCliDaemonConfig {
    fn get_task_state_dir(&self) -> &PathBuf {
        &DEFAULT_PATH_BUF
    }
}

impl ProvidesServerDetails for SyncCliDaemonConfig {
    fn get_server_url(&self) -> &String {
        &self.server_url
    }

    fn get_server_secret_value(&self) -> String {
        self.sync_secret.clone()
    }
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn sync_local_store_with_server(config_raw: &JsValue) -> Result<(), JsValue> {
    let config: SyncCliDaemonConfig = config_raw
        .into_serde()
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Parse).from(e)
        })
        .map_err::<JsValue, _>(HyperTaskError::into)?;

    let _ = sync_all_tasks(&config).map_err::<JsValue, _>(HyperTaskError::into)?;

    Ok(())
}
