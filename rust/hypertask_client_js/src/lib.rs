extern crate js_sys;
extern crate lazy_static;

//use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[cfg(target_arch = "wasm32")]
pub fn get_stack_trace(
    _cmd_raw: &JsValue,
    _updater_fn: &js_sys::Function,
    _input_iter_raw: &JsValue,
    _stack_machine_program_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    use hypertask_engine::prelude::*;
    use std::collections::HashMap;
    use std::rc::Rc;
    use wasm_bindgen::prelude::*;

    let _input_tasks: HashMap<Rc<Id>, Rc<Task>> =
        hypertask_task_io_operations::get_input_tasks(&())
            .map_err::<JsValue, _>(HyperTaskError::into)?;

    Ok(JsValue::from_str("test"))

    //let response: Vec<ScoredTask> = hypertask_engine::prelude::run(cmd, context)
    //.map_err(|e| format!("Error running hypertask engine: {}", e))?;

    //Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
