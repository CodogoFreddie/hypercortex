use chrono::prelude::*;

#[derive(Debug)]
pub enum Prop {
    Description(String),
    Done(DateTime<Utc>),
    Due(DateTime<Utc>),
    Snooze(DateTime<Utc>),
    Wait(DateTime<Utc>),
}
