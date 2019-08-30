use chrono::prelude::*;
use hypertask_engine::prelude::*;
use rand::prelude::*;
use rand::seq::IteratorRandom;
use wasm_bindgen::prelude::*;

pub struct WebTaskIterator {
    input_iter_from_js: js_sys::IntoIter,
}

impl WebTaskIterator {
    pub fn new(input_iter_raw: &JsValue) -> HyperTaskResult<Self> {
        //TODO get rid of these unwraps
        let input_iter_from_js = js_sys::try_iter(input_iter_raw).unwrap().unwrap();
        Ok(Self { input_iter_from_js })
    }
}

impl Iterator for WebTaskIterator {
    type Item = HyperTaskResult<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        let next_item: Option<HyperTaskResult<Task>> =
            self.input_iter_from_js
                .next()
                .map(|x: Result<JsValue, JsValue>| {
                    x.map_err(|e| {
                        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    })
                    .and_then(|js_val| {
                        let x: Result<Task, HyperTaskError> = js_val.into_serde().map_err(|e| {
                            HyperTaskError::new(
                                HyperTaskErrorDomain::Task,
                                HyperTaskErrorAction::Read,
                            )
                        });

                        x
                    })
                });

        next_item
    }
}

//------------------------------

pub struct WebContext<'a> {
    updater_fn: &'a js_sys::Function,
    rng: SmallRng,
    input_iter_raw: &'a JsValue,
}

impl<'a> WebContext<'a> {
    pub fn new(updater_fn: &'a js_sys::Function, input_iter_raw: &'a JsValue) -> Self {
        let epoch_milis = js_sys::Date::now();

        Self {
            rng: SmallRng::seed_from_u64(epoch_milis as u64),
            updater_fn,
            input_iter_raw,
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
    fn put_task(&mut self, task: &Task) -> HyperTaskResult<()> {
        let js_task = JsValue::from_serde(task).unwrap();

        self.updater_fn
            .call1(&JsValue::null(), &js_task)
            .map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    .with_msg(|| format!("Could not update task `{}`", task.get_id()))
            })?;

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

impl<'a> GetTaskIterator for WebContext<'a> {
    type TaskIterator = WebTaskIterator;

    fn get_task_iterator(&self) -> HyperTaskResult<Self::TaskIterator> {
        WebTaskIterator::new(self.input_iter_raw)
    }
}
