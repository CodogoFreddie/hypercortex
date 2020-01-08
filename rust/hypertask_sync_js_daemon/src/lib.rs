#[macro_use]
extern crate lazy_static;

use hypertask_engine::prelude::*;
use hypertask_sync_storage_with_server::*;
use hypertask_task_io_operations::ProvidesDataDir;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

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

fn get_local_storage() -> HyperTaskResult<web_sys::Storage> {
    let window = web_sys::window().ok_or(
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
            .msg("can't get window"),
    )?;

    window
        .local_storage()
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("can't get local_storage")
        })?
        .ok_or(
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("can't get local_storage"),
        )
}

async fn sync_all_tasks_async_wrapper() -> Result<JsValue, JsValue> {
    let local_storage = get_local_storage().expect("could not get local storage");

    let serial_config = local_storage
        .get(&format!("hypertask::config"))
        .expect("could not read config")
        .expect("could not read config");
    let config: SyncCliDaemonConfig =
        serde_json::from_str(&serial_config).expect("could not parse config");

    let result: Result<(), HyperTaskError> = sync_all_tasks_async(&config).await;

    return result
        .map_err(HyperTaskError::into)
        .map(|_| JsValue::from_str("ok!"));
}

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn sync_local_store_with_server(config_raw: &JsValue) -> Result<Promise, JsValue> {
    use wasm_bindgen::prelude::*;

    console_error_panic_hook::set_once();

    let sync_future = sync_all_tasks_async_wrapper();
    let sync_promise = future_to_promise(sync_future);
    Ok(sync_promise)
}
