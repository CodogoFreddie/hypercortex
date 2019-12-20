extern crate js_sys;
extern crate lazy_static;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn get_stack_trace(
    cmd_raw: &JsValue,
    updater_fn: &js_sys::Function,
    input_iter_raw: &JsValue,
    stack_machine_program_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let input_tasks: HashMap<Rc<Id>, Rc<Task>> = hypertask_task_io_operations::get_input_tasks()
        .map_err::<JsValue, _>(HyperTaskError::into)?;

    Ok(JsValue::from_str("test"))

    //let response: Vec<ScoredTask> = hypertask_engine::prelude::run(cmd, context)
    //.map_err(|e| format!("Error running hypertask engine: {}", e))?;

    //Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
