use chrono::prelude::*;

#[derive(Debug)]
pub enum Prop {
    Description(String),
    Due(DateTime<Utc>),
}
