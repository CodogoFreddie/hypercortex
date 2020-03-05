use hypertask_engine::prelude::*;
use serde::{Deserialize, Serialize};
use simple_persist_data::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
struct ClientTask(Task);

impl PersistableMultiple for ClientTask {
    const APP_DATA_TYPE: AppDataType = AppDataType::UserData;
    const FORMAT: Format = Format::Json;
    const NAME: &'static str = "task";

    const APP_INFO: AppInfo = AppInfo {
        name: "hypertask-sync-client".into(),
        author: "hypertask".into(),
    };
}
