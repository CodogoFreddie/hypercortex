use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveParsingError {
    MalformedBoolean(String),
    MalformedDateLike(String),
    UnknownProp(String),
}

impl Error for PrimitiveParsingError {}

impl fmt::Display for PrimitiveParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PrimitiveParsingError::*;

        match self {
            MalformedBoolean(token) => write!(f, "the prop `{}` is not a valid boolean", token),
            MalformedDateLike(token) => write!(
                f,
                "the prop `{}` is not a valid date/time or date/time shortcut",
                token
            ),
            UnknownProp(token) => write!(f, "the prop `{}` is not a known prop", token),
        }
    }
}
