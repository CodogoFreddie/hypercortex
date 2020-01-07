use hypertask_engine::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;

pub trait ProvidesDataDir: Sync + Send {
    fn get_task_state_dir(&self) -> &PathBuf;
}

#[cfg(target_arch = "wasm32")]
mod wasm;
#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
mod cli;
#[cfg(not(target_arch = "wasm32"))]
pub use cli::*;
