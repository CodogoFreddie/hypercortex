use std::cmp::{Eq, PartialEq};

#[derive(Debug)]
pub enum Sign {
    Plus,
    Minus,
}

#[derive(Debug)]
pub struct Tag {
    content: String,
    sign: Sign,
}

impl Tag {
    fn new(source: String) -> Result<Self, String> {
        let sign = match string.chars().next().unwrap() {
            '+' => Sign::Plus,
            '-' => Sign::Plus,
            _ => return Err(format!("can not parse tag {}", source)),
        };

        Self {
            content: source[1..],
            sign,
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.content.contains(other.content.as_str())
            || other.content.contains(self.content.as_str())
    }
}

impl Eq for Tag {}
