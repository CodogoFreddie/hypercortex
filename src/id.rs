use rand::seq::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::cmp::{Eq, PartialEq};
use std::fmt;

const CHARS: &'static str = "23456789abcdefghkmnpqrstwxyz";
pub const NUMBER_OF_CHARS_IN_FULL_ID: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id(pub String);

impl Id {
    pub fn generate() -> Self {
        Self(Id::get_easy_type_id())
    }

    fn get_easy_type_id() -> String {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = CHARS
                .chars()
                .choose(&mut rand::thread_rng())
                .expect("Couldn't get random char");

            result.push(random);
        }

        result
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        let Id(self_content) = self;
        let Id(other_content) = other;

        self_content.contains(other_content.as_str())
            || other_content.contains(self_content.as_str())
    }
}

impl Eq for Id {}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Id(x) = self;
        write!(f, "{}", x)
    }
}
