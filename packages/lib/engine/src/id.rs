use rand::seq::IteratorRandom;
use rand::thread_rng;
use serde::{Deserialize, Serialize};

use std::fmt;

pub const VALID_ID_CHARS: &str = "23456789abcdefghkmnpqrstwxyz";
pub const NUMBER_OF_CHARS_IN_FULL_ID: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct Id(pub String);

impl Id {
    pub fn generate() -> Self {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = VALID_ID_CHARS
                .chars()
                .choose(&mut thread_rng())
                .expect("Couldn't get random char");

            result.push(random);
        }

        Id(result)
    }

    pub fn sub_eq(&self, other: &Id) -> bool {
        let Id(self_content) = self;
        let Id(other_content) = other;

        self_content.contains(other_content.as_str())
            || other_content.contains(self_content.as_str())
    }
}

impl AsRef<str> for Id {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Id(x) = self;
        write!(f, "{}", x)
    }
}

impl simple_persist_data::prelude::PersistanceId for Id {
    fn as_path_section(&self) -> &str {
        self.0.as_str()
    }

    fn from_path_section(s: &str) -> Self {
        Id(s.to_owned())
    }
}
