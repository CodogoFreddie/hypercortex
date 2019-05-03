use crate::task::id::Id;
use crate::task::tag::Tag;

#[derive(Debug, Eq, PartialEq)]
pub enum Query {
    All,
    Id(Id),
    New,
    Tag(Tag),
}
