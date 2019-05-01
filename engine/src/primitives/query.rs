use super::id::Id;
use super::parsing_error::{PrimitiveParsingError, PrimitiveParsingResult};
use super::tag::Tag;

#[derive(Debug, Eq, PartialEq)]
pub enum Query {
    Id(Id),
    Tag(Tag),
}

impl Query {
    pub fn from_string(string: &str) -> PrimitiveParsingResult<Self> {
        Tag::from_string(string) //try to parse as a Tag
            .and_then(|tag| Ok(Query::Tag(tag))) //and if successful: wrap in a Query
            .or_else(|_| Ok(Query::Id(Id::new(string)))) //otherwise, it's an Id
    }
}
