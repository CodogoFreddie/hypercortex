use super::{Prop, Tag};

#[derive(Debug, Eq, PartialEq)]
pub enum Mutation {
    AddTag(Tag),
    RemoveTag(Tag),
    SetProp(Prop),
}
