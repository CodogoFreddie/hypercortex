use crate::recur::Recur;
use chrono::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub enum Prop {
    Description(String),
    Done(DateTime<Utc>),
    Due(Option<DateTime<Utc>>),
    Recur(Option<Recur>),
    Snooze(Option<DateTime<Utc>>),
    Wait(Option<DateTime<Utc>>),
}
