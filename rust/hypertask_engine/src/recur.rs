use serde::{Deserialize, Serialize};
use std::fmt;
use time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Hash)]
pub enum Recur {
    Day(i64),
    Week(i64),
    Month(i64),
    Year(i64),
}

impl From<&Recur> for Duration {
    fn from(recur: &Recur) -> Self {
        match recur {
            Recur::Day(n) => Duration::days(*n),
            Recur::Week(n) => Duration::weeks(*n),
            Recur::Month(n) => Duration::seconds(n * 60 * 60 * 24 * 365 / 12),
            Recur::Year(n) => Duration::days(n * 365),
        }
    }
}

impl fmt::Display for Recur {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Recur::Day(n) => write!(f, "{} days", n),
            Recur::Week(n) => write!(f, "{} weeks", n),
            Recur::Month(n) => write!(f, "{} months", n),
            Recur::Year(n) => write!(f, "{} years", n),
        }
    }
}
