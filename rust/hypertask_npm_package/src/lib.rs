#[macro_use]
extern crate lazy_static;
extern crate js_sys;

<<<<<<< HEAD
=======
mod context;

use crate::context::{WebContext, WebTaskIterator};
>>>>>>> better-cli-config
use chrono::prelude::*;
use hypertask_engine::prelude::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use wasm_bindgen::prelude::*;

<<<<<<< HEAD
struct WebContext<'a> {
    updater_fn: &'a js_sys::Function,
    rng: SmallRng,
}

impl<'a> WebContext<'a> {
    pub fn new(updater_fn: &'a js_sys::Function) -> Self {
        let epoch_milis = js_sys::Date::now();

        Self {
            rng: SmallRng::seed_from_u64(epoch_milis as u64),
            updater_fn,
        }
    }
}

impl<'a> GetNow for WebContext<'a> {
    fn get_now(&self) -> DateTime<Utc> {
        let iso_string: String = js_sys::Date::new_0().to_iso_string().as_string().unwrap();

        let fixed_offset: DateTime<FixedOffset> =
            DateTime::parse_from_rfc3339(&iso_string).unwrap();

        DateTime::<Utc>::from(fixed_offset)
    }
}

impl<'a> PutTask for WebContext<'a> {
    fn put_task(&mut self, task: &Task) -> Result<(), String> {
        let js_task = JsValue::from_serde(task).unwrap();

        self.updater_fn
            .call1(&JsValue::null(), &js_task)
            .map_err(|_| format!("Could not update task `{}`", task.get_id()))?;

        Ok(())
    }
}

impl<'a> GenerateId for WebContext<'a> {
    fn generate_id(&mut self) -> String {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = VALID_ID_CHARS
                .chars()
                .choose(&mut self.rng)
                .expect("Couldn't get random char");

            result.push(random);
        }

        result
    }
}

=======
>>>>>>> better-cli-config
//#[wasm_bindgen(start)]
#[wasm_bindgen]
pub fn run(
    cmd_raw: &JsValue,
    updater_fn: &js_sys::Function,
    input_iter_raw: &JsValue,
) -> Result<JsValue, JsValue> {
<<<<<<< HEAD
    let input_iter = js_sys::try_iter(input_iter_raw)?
        .ok_or_else(|| "need to pass iterable JS values!")?
        .map(|x| {
            x.and_then(|js_val| {
                js_val
                    .into_serde()
                    .map_err(|_| JsValue::from_str("Error parsing task"))
            })
            .map_err(|_| "Error getting task".to_owned())
        });

    let context = WebContext::new(updater_fn);
=======
    let context = WebContext::new(updater_fn, input_iter_raw);
>>>>>>> better-cli-config

    let cmd: Command = cmd_raw
        .into_serde()
        .map_err(|_| JsValue::from_str("could not parse input command"))?;

<<<<<<< HEAD
    let response: Vec<FinalisedTask> = hypertask_engine::prelude::run(cmd, context, input_iter)
=======
    let response: Vec<FinalisedTask> = hypertask_engine::prelude::run(cmd, context)
>>>>>>> better-cli-config
        .map_err(|_| "Error running hypertask engine".to_owned())?;

    Ok(JsValue::from_serde(&response).map_err(|_| "Error stringifying output")?)
}
