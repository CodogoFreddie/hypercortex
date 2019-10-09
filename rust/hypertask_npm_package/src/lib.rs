extern crate js_sys;
extern crate lazy_static;

mod context;

use crate::context::WebContext;
use hypertask_engine::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(
    cmd_raw: &JsValue,
    updater_fn: &js_sys::Function,
    input_iter_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let context = WebContext::new(updater_fn, input_iter_raw);

    let cmd: Command = cmd_raw.into_serde().map_err(|e| {
        JsValue::from_str(
            format!("[{:?}] ({:?}) could not parse input command", cmd_raw, e).as_str(),
        )
    })?;

    let response: Vec<FinalisedTask> = hypertask_engine::prelude::run(cmd, context)
        .map_err(|e| format!("Error running hypertask engine: {}", e))?;

    Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
