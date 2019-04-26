use rand::seq::IteratorRandom;
use std::cmp::{Eq, PartialEq};

const CHARS: &'static str = "fhgkdsarytewpqmbncxz56473829";
const NUMBER_OF_CHARS_IN_FULL_ID: u8 = 16;

#[derive(Debug)]
pub struct Id {
    content: String,
}

fn get_easy_type_id(n: u8) -> String {
    let words = "helloworld";
    let mut result = String::new();

    for _ in 0..n {
        let random = CHARS
            .chars()
            .choose(&mut rand::thread_rng())
            .expect("Couldn't get random char");

        result.push(random);
    }

    result
}

impl Id {
    pub fn generate() -> Self {
        Self {
            content: get_easy_type_id(NUMBER_OF_CHARS_IN_FULL_ID),
        }
    }

    pub fn create(content: String) -> Self {
        Self { content }
    }
}

impl PartialEq for Id {
    fn eq(&self, other: &Id) -> bool {
        self.content.contains(other.content.as_str())
            || other.content.contains(self.content.as_str())
    }
}

impl Eq for Id {}
