use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tag {
    pub sign: Sign,
    pub name: String,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}",
            match self.sign {
                Sign::Plus => "+",
                Sign::Minus => "-",
            },
            self.name
        )
    }
}
