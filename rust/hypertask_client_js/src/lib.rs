#![deny(clippy::all)]
#![deny(clippy::option_unwrap_used, clippy::result_unwrap_used)]

extern crate js_sys;
extern crate lazy_static;

use hypertask_engine::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn get_machine_stack_trace(
    test_task_raw: &JsValue,
    programs_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let test_task: Task = test_task_raw
        .into_serde()
        .map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;
    let programs_string_chunks: Vec<String> = programs_raw
        .into_serde()
        .map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;

    let program = RPNSymbol::parse_programs(&programs_string_chunks);

    let mut env = HashMap::new();
    env.insert("now", 1234.0);

    let mut machine = StackMachine::new(program, env);

    let trace = machine
        .run_with_snapshots(&test_task, &HashMap::new())
        .expect("could not run");

    Ok(JsValue::from_serde(
        &trace
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .map(RPNSymbol::stringify)
                    .collect::<Vec<String>>()
            })
            .collect::<Vec<Vec<String>>>(),
    )
    .map_err(|_| "bad")?)
}
