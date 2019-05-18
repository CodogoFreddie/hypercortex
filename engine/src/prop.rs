use crate::period::Period;
use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Prop {
    Description(String),
    Due(Option<DateTime<Utc>>),
    Recur(Option<Period>),
}
