use chrono::prelude::*;

#[derive(Debug)]
pub enum Prop {
    Description(String),
    Done(DateTime<Utc>),
    Due(Option<DateTime<Utc>>),
    Snooze(Option<DateTime<Utc>>),
    Wait(Option<DateTime<Utc>>),
}
