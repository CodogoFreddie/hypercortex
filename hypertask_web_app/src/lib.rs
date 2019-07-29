#[macro_use]
extern crate lazy_static;
extern crate js_sys;

use chrono::prelude::*;
use hypertask_engine::prelude::*;
use wasm_bindgen::prelude::*;

pub fn register_updater(updater_fn: &js_sys::Function) -> () {}

struct WebContext<'a> {
    updater_fn: &'a js_sys::Function,
    input_iter: js_sys::IntoIter,
}

impl<'a, TaskIterator> Context<TaskIterator> for WebContext<'a>
where
    TaskIterator: Iterator<Item = Result<Task, String>>,
{
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }

    fn get_input_tasks_iter(&self) -> impl TaskIterator {
        let vec: Vec<Result<Task, String>> = vec![];

        vec.iter()
        //self.input_iter
        //.map(|r: Result<JsValue, JsValue>| match r {
        //Ok(task) => Ok(task.into_serde().unwrap()),
        //Err(e) => Err("Error".to_owned()),
        //})
        //.collect::<Vec<Result<Task, String>>>()
        //.iter()
    }

    fn put_task(&self, task: &Task) -> Result<(), String> {
        let js_task = JsValue::from_serde(task).unwrap();

        self.updater_fn.call1(&JsValue::null(), &js_task);

        Ok(())
    }
}

//#[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run(
    cmd_raw: &JsValue,
    updater_fn: &js_sys::Function,
    input_iter_raw: &JsValue,
) -> Result<JsValue, JsValue> {
    let input_iter =
        js_sys::try_iter(input_iter_raw)?.ok_or_else(|| "need to pass iterable JS values!")?;

    let context = WebContext {
        input_iter,
        updater_fn,
    };

    let cmd: Command = cmd_raw
        .into_serde()
        .map_err(|e| JsValue::from_str("could not parse input command"))?;

    let response: Vec<FinalisedTask> = hypertask_engine::prelude::run(cmd, &context).unwrap();

    Ok(JsValue::from_serde(&response).unwrap())
}
