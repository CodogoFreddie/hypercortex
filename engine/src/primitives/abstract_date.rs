use chrono::prelude::*;
use std::cmp::{Eq, PartialEq};
use std::fmt;

pub enum AbstractDate {
    Definite(DateTime<Utc>),
    Deferred(Box<Fn(&[DateTime<Utc>]) -> DateTime<Utc>>),
}

impl AbstractDate {
    fn resolve(self, dates: &[DateTime<Utc>]) -> DateTime<Utc> {
        match self {
            AbstractDate::Definite(x) => x,
            AbstractDate::Deferred(func) => func(dates),
        }
    }
}

impl Eq for AbstractDate {}

impl PartialEq for AbstractDate {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AbstractDate::Definite(x), AbstractDate::Definite(y)) => x == y,
            _ => false,
        }
    }
}

impl fmt::Debug for AbstractDate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AbstractDate::Definite(x) => write!(f, "AbstractDate::Definite({})", x),
            AbstractDate::Deferred(_) => write!(f, "AbstractDate::Deferred"),
        }
    }
}
