use std::error::Error;
use std::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum CortexError {
    Del(IoError),
    Mount(IoError),
    Put(IoError),
    Setup(IoError),
}

impl Error for CortexError {
    fn cause(&self) -> Option<&Error> {
        use CortexError::*;

        match *self {
            CortexError::Del(ref e) => Some(e),
            CortexError::Mount(ref e) => Some(e),
            CortexError::Put(ref e) => Some(e),
            CortexError::Setup(ref e) => Some(e),
        }
    }
}

impl fmt::Display for CortexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use CortexError::*;

        match self {
            Del(cause) => write!(f, "CortexError::Del({})", cause),
            Mount(cause) => write!(f, "CortexError::Mount({})", cause),
            Put(cause) => write!(f, "CortexError::Put({})", cause),
            Setup(cause) => write!(f, "CortexError::Setup({})", cause),
        }
    }
}

pub type CortexResult<T> = Result<T, CortexError>;
