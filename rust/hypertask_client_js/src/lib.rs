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
    initial_commands_raw: &JsValue,
    stack_values_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let test_task: Task = test_task_raw
        .into_serde()
        .map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;
    let initial_commands: String = initial_commands_raw.as_string().unwrap();
    let program_chunks: Vec<String> = stack_values_raw
        .into_serde()
        .map_err(|e| JsValue::from_str(format!("{}", e).as_str()))?;

    let mut commands = RPNSymbol::parse_program(&initial_commands);
    let mut main_commands = RPNSymbol::parse_programs(&program_chunks);

    commands.append(&mut main_commands);

    let mut env = HashMap::new();
    env.insert("now", 1234.0);

    let mut machine = StackMachine::new(commands, env);

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
