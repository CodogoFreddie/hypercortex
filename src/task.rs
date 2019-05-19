use crate::id::Id;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub created_at: DateTime<Utc>,
    pub id: Id,
}
