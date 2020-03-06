use hypertask_engine::prelude::*;
use serde::{Deserialize, Serialize};
use simple_persist_data::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug)]
pub struct PersistedTaskClient(pub Task);

impl PersistableMultiple for PersistedTaskClient {
    const APP_DATA_TYPE: AppDataType = AppDataType::UserData;
    const APP_INFO: AppInfo = app_info_client::APP_INFO;
    const FORMAT: simple_persist_data::Format = Format::Json;
    const NAME: &'static str = "task";

    type ID = Id;

    fn get_id(&self) -> &Self::ID {
        &self.0.get_id_ref()
    }
}
