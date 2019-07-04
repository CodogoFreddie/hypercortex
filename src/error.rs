use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CortexError {
    ContextWrapper { msg: String, source: Box<Error> },
}

impl Error for CortexError {}

impl fmt::Display for CortexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CortexError::ContextWrapper { msg, source } => write!(f, "{} ({})", msg, source),
        }
    }
}
