use super::parsing_error::{PrimitiveParsingError, PrimitiveParsingResult};
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
    pub fn from_string(source_str: &str) -> PrimitiveParsingResult<Self> {
        let mut source = String::from(source_str);

        let sign = match source.remove(0) {
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            _ => return Err(PrimitiveParsingError::MalformedTag(source)),
        };

        Ok(Self {
            content: source,
            sign,
        })
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_construct_from_string() {
        let plus_foo = Tag::from_string("+foo");

        assert_eq!(plus_foo, Ok(Tag::new("foo", Sign::Plus,)));

        let minus_bar = Tag::from_string("-bar");

        assert_eq!(minus_bar, Ok(Tag::new("bar", Sign::Minus,)));
    }
}
