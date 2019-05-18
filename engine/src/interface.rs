use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Mutation {
    Prop(Prop),
    Tag(Tag),
    Delete,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Query {
    All,
    Id(Id),
    New,
    Tag(Tag),
}
