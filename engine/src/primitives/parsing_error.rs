use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveParsingError {
    UnknownProp(String),
    MalformedBoolean(String),
}

impl Error for PrimitiveParsingError {}

impl fmt::Display for PrimitiveParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PrimitiveParsingError::*;

        match self {
            UnknownProp(token) => write!(f, "the prop `{}` is not a known prop", token),
            MalformedBoolean(token) => write!(f, "the prop `{}` is not a valid boolean", token),
        }
    }
}
