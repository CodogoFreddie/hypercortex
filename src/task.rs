use crate::id::Id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: Id,
}

impl Task {
    pub fn get_id(&self) -> &Id {
        &(self.id)
    }
}
