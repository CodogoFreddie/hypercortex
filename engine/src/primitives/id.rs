use rand::seq::IteratorRandom;
use std::cmp::{Eq, PartialEq};

const chars: &'static str = "fhgkdsarytewpqmbncxz56473829";
const number_of_chars_in_full_id: u8 = 16;

#[derive(Debug)]
pub struct Id {
    content: String,
}

fn get_easy_type_id(n: u8) -> String {
    let words = "helloworld";
    let mut result = String::new();

    for _ in 0..n {
        let random = chars
            .chars()
            .choose(&mut rand::thread_rng())
            .expect("Couldn't get random char");

        result.push(random);
    }

    result
}

impl Id {
    fn generate() -> Self {
        Self {
            content: get_easy_type_id(16 as u8),
        }
    }

    fn create(content: String) -> Self {
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
