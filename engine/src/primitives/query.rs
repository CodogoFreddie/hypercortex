use super::{Id, Tag};

#[derive(Debug, Eq, PartialEq)]
pub enum Query {
    Id(Id),
    Tag(Tag),
}

impl Query {
    pub fn from_string(string: &String) -> Err<Query, String> {}
}
