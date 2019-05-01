use std::{error::Error, fmt};

#[derive(Debug, Eq, PartialEq)]
pub enum PrimitiveParsingError {
    MalformedBoolean(String),
    MalformedDateLike(String),
    MalformedPeriod(String),
    MalformedRecur(String),
    NotAPartialIsoDate(String),
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
            MalformedPeriod(token) => write!(f, "the period `{}` is not in a valid format", token),
            MalformedRecur(token) => {
                write!(f, "the prop `{}` is not in a valid recur format", token)
            }
            NotAPartialIsoDate(token) => write!(f, "{} is not a partial ISO date", token),
            UnknownProp(token) => write!(f, "the prop `{}` is not a known prop", token),
        }
    }
}

pub type PrimitiveParsingResult<T> = Result<T, PrimitiveParsingError>;
