use hypertask_engine::prelude::*;
use serde::{Deserialize, Serialize};
use simple_persist_data::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct CliStoredTask(Task);

pub fn get_input_tasks() -> HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>> {
    let mut hm = HashMap::new();

    for id in CliStoredTask::get_all_ids()? {
        let CliStoredTask(task) = CliStoredTask::load_from_storage(&id)?;
        hm.insert(Rc::new(id), Rc::new(task));
    }

    Ok(hm)
}

pub fn put_output_tasks(task: Task) -> HyperTaskResult<()> {
    CliStoredTask(task).save_to_storage()?;
    Ok(())
}

impl PersistableMultiple for CliStoredTask {
    const APP_DATA_TYPE: AppDataType = AppDataType::UserData;
    const APP_INFO: AppInfo = crate::app_info::APP_INFO;
    const FORMAT: simple_persist_data::Format = Format::Json;
    const NAME: &'static str = "task";

    type ID = Id;

    fn get_id(&self) -> &Self::ID {
        &self.0.get_id_ref()
    }
}
