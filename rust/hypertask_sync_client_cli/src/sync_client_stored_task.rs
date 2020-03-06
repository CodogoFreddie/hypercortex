use hypertask_engine::prelude::*;
use serde::{Deserialize, Serialize};
use simple_persist_data::prelude::*;

pub const APP_INFO: AppInfo = AppInfo {
    name: "hypertask-cli",
    author: "hypertask",
};

#[derive(Serialize, Deserialize, Debug)]
struct SyncClientStoredTask(Task);

impl PersistableMultiple for SyncClientStoredTask {
    const APP_DATA_TYPE: AppDataType = AppDataType::UserData;
    const APP_INFO: AppInfo = crate::app_info::APP_INFO;
    const FORMAT: simple_persist_data::Format = Format::Json;
    const NAME: &'static str = "task";

    type ID = Id;

    fn get_id(&self) -> &Self::ID {
        &self.0.get_id_ref()
    }
}
