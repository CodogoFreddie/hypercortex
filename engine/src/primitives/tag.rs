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
    pub fn new(mut source: String) -> Result<Self, String> {
        let sign = match source.remove(0) {
            '+' => Sign::Plus,
            '-' => Sign::Minus,
            _ => return Err(format!("can not parse tag {}", source)),
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
        let plus_foo = Tag::new(String::from("+foo"));

        assert_eq!(
            plus_foo,
            Ok(Tag {
                sign: Sign::Plus,
                content: String::from("foo")
            })
        );

        let minus_bar = Tag::new(String::from("-bar"));

        assert_eq!(
            minus_bar,
            Ok(Tag {
                sign: Sign::Minus,
                content: String::from("bar")
            })
        );
    }
}
