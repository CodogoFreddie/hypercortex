use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PrimitiveParsingError {
    PropBool(&'static str, String),
    TooManyColons(String),
    Unknown(String),
}

impl Error for PrimitiveParsingError {}

impl fmt::Display for PrimitiveParsingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use PrimitiveParsingError::*;

        match self {
            PropBool(key, value) => write!(
                f,
                "the prop `{}:{}` could not be parsed, is it formatted correctly?",
                key, value
            ),
            TooManyColons(token) => write!(f, "the prop `{}` contains too many colons", token),
            Unknown(token) => write!(f, "the prop `{}` is not a known prop", token),
        }
    }
}
