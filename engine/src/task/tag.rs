use std::cmp::{Eq, PartialEq};

#[derive(Debug, Eq, PartialEq)]
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
    pub fn new(content: &str, sign: Sign) -> Self {
        Self {
            content: String::from(content),
            sign,
        }
    }
}

impl PartialEq for Tag {
    fn eq(&self, other: &Tag) -> bool {
        self.sign == other.sign
            && (self.content.contains(other.content.as_str())
                || other.content.contains(self.content.as_str()))
    }
}
impl Eq for Tag {}
