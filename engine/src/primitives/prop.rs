use super::parsing_error::PrimitiveParsingError;
use super::Period;
use chrono::prelude::*;

#[derive(Debug, Eq, PartialEq)]
pub enum Prop {
    Archived(bool),
    Canonical(String),
    CreatedAt(DateTime<Utc>),
    Description(String),
    Done(DateTime<Utc>),
    Due(DateTime<Utc>),
    Icon(String),
    Image(String),
    Keywords(Vec<String>),
    Recur(Period),
    SiteName(String),
    Snooze(DateTime<Utc>),
    Subject(String),
    MetaTags(Vec<String>),
    Title(String),
    Url(String),
    Wait(DateTime<Utc>),
}

impl Prop {
    fn parse_boolean(name: &'static str, value: &String) -> Result<Self, PrimitiveParsingError> {
        match value.as_ref() {
            "true" => Ok(Prop::Archived(true)),
            "false" => Ok(Prop::Archived(false)),
            _ => Err(PrimitiveParsingError::PropBool(name, value.clone())),
        }
    }

    //fn parse_date_time(name: &'static str, value: &String) -> Result<Self, PrimitiveParsingError> {

    //}

    /// tries to parse a string to a prop
    /// returns None if the string is not a prop
    /// returns Some(Err) if the string is a malformed prop
    /// returns Some(Ok) if the string parsed correctly
    pub fn from_string(string: String) -> Option<Result<Self, PrimitiveParsingError>> {
        let mut string_parts: Vec<String> = string.split(":").map(|x| String::from(x)).collect();

        if (string_parts.len() < 2) {
            return None;
        }
        if (string_parts.len() > 2) {
            return Some(Err(PrimitiveParsingError::TooManyColons(string)));
        }

        let value = string_parts.pop().unwrap();
        let key = string_parts.pop().unwrap();

        let parsed = match (key.as_ref(), &value) {
            ("canonical", value) => Ok(Prop::Canonical(value.clone())),
            ("description", value) => Ok(Prop::Description(value.clone())),

            ("archived", value) => Prop::parse_boolean("archived", value),
            ("done", value) => Prop::parse_done(value),
            //("due", value) => Prop::parse_due(value),
            //("icon", value) => Prop::parse_icon(value),
            //("image", value) => Prop::parse_image(value),
            //("keywords", value) => Prop::parse_keywords(value),
            //("recur", value) => Prop::parse_recur(value),
            //("sitename", value) => Prop::parse_sitename(value),
            //("snooze", value) => Prop::parse_snooze(value),
            //("subject", value) => Prop::parse_subject(value),
            //("tags", value) => Prop::parse_meta_tags(value),
            //("title", value) => Prop::parse_title(value),
            //("url", value) => Prop::parse_url(value),
            //("wait", value) => Prop::parse_wait(value),
            (_, _) => Err(PrimitiveParsingError::Unknown(string)),
        };

        Some(parsed)
    }
}
