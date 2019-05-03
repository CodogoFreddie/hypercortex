use super::super::task::Tag;
use super::Prop;
use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Mutation {
    Prop(Prop),
    Tag(Tag),
    Delete,
}
