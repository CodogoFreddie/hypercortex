#[macro_use]
extern crate lazy_static;
extern crate js_sys;

use chrono::prelude::*;
use hypertask_engine::prelude::*;
use wasm_bindgen::prelude::*;

pub fn register_updater(updater_fn: &js_sys::Function) -> () {}

struct WebContext<'a> {
    updater_fn: &'a js_sys::Function,
}

impl<'a> GetNow for WebContext<'a> {
    fn get_now(&self) -> DateTime<Utc> {
        Utc.ymd(2014, 7, 8).and_hms(9, 10, 11)
    }
}

impl<'a> PutTask for WebContext<'a> {
    fn put_task(&mut self, task: &Task) -> Result<(), String> {
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
    let input_iter = js_sys::try_iter(input_iter_raw)?
        .ok_or_else(|| "need to pass iterable JS values!")?
        .map(|x| {
            x.and_then(|jsVal| {
                jsVal
                    .into_serde()
                    .map_err(|_| JsValue::from_str("Error parsing task"))
            })
            .map_err(|_| "Error getting task".to_owned())
        });

    let context = WebContext { updater_fn };

    let cmd: Command = cmd_raw
        .into_serde()
        .map_err(|e| JsValue::from_str("could not parse input command"))?;

    let response: Vec<FinalisedTask> = hypertask_engine::prelude::run(cmd, context, input_iter)
        .map_err(|_| "Error running hypertask engine".to_owned())?;

    Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
