use super::parsing_error::PrimitiveParsingError;
use super::{Prop, Tag};
use chrono::prelude::*;
use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum Mutation {
    PlainText(String),
    Prop(Prop),
    Tag(Tag),
}

impl Mutation {
    pub fn from_string(
        get_now: &Fn() -> DateTime<Utc>,
        string: &str,
    ) -> Result<Mutation, PrimitiveParsingError> {
        let try_tag = Tag::from_string(string);

        if let Ok(tag) = try_tag {
            return Ok(Mutation::Tag(tag));
        }

        let try_prop = Prop::from_string(get_now, &string[..]);

        match try_prop {
            None => Ok(Mutation::PlainText(String::from(string))),
            Some(Ok(prop)) => Ok(Mutation::Prop(prop)),
            Some(Err(e)) => Err(e),
        }
    }
}
