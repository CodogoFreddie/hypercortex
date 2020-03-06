extern crate js_sys;
extern crate lazy_static;

mod context;

use crate::context::WebContext;
use hypertask_engine::prelude::*;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run(
    cmd_raw: &JsValue,
    updater_fn: &js_sys::Function,
    input_iter_raw: &JsValue,
    stack_machine_program_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let stack_machine_program = stack_machine_program_raw.as_string().unwrap();

    let context = WebContext::new(updater_fn, input_iter_raw, stack_machine_program);

    let cmd: Command = cmd_raw.into_serde().map_err(|e| {
        JsValue::from_str(
            format!("[{:?}] ({:?}) could not parse input command", cmd_raw, e).as_str(),
        )
    })?;

    let response: Vec<ScoredTask> = hypertask_engine::prelude::run(cmd, context)
        .map_err(|e| format!("Error running hypertask engine: {}", e))?;

    Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
