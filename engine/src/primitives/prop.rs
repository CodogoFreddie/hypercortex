use super::Period;
use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Prop {
    Description(String),
    Due(DateTime<Utc>),
    Wait(DateTime<Utc>),
    Done(DateTime<Utc>),
    Snooze(DateTime<Utc>),
    CreatedAt(DateTime<Utc>),
    Recur(Period),
}
