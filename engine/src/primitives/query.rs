use super::{Id, Tag};

#[derive(Debug, Eq, PartialEq)]
pub enum Query {
    Id(Id),
    Tag(Tag),
}

impl Query {
    pub fn from_string(string: String) -> Query {
        match Tag::from_string(string.clone()) {
            Ok(tag) => Query::Tag(tag),
            Err(e) => Query::Id(Id::new(string)),
        }
    }
}
