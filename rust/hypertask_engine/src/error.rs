use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HyperTaskErrorDomain {
    Task,
    Config,
    Context,
}

impl fmt::Display for HyperTaskErrorDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HyperTaskErrorDomain::Config => "config",
                HyperTaskErrorDomain::Context => "context",
                HyperTaskErrorDomain::Task => "task",
            }
        )
    }
}

#[derive(Debug)]
pub enum HyperTaskErrorAction {
    Create,
    Read,
    Write,
    Delete,
}

impl fmt::Display for HyperTaskErrorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HyperTaskErrorAction::Read => "read",
                HyperTaskErrorAction::Write => "write",
                HyperTaskErrorAction::Create => "create",
                HyperTaskErrorAction::Delete => "delete",
            }
        )
    }
}

#[derive(Debug)]
pub struct HyperTaskError {
    domain: HyperTaskErrorDomain,
    action: HyperTaskErrorAction,
    meta: Option<&'static str>,
}

pub type HyperTaskResult<T> = Result<T, HyperTaskError>;

impl HyperTaskError {
    pub fn new(
        domain: HyperTaskErrorDomain,
        action: HyperTaskErrorAction,
        meta: Option<&'static str>,
    ) -> Self {
        Self {
            domain,
            action,
            meta,
        }
    }
}

impl fmt::Display for HyperTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HyperTaskError [ {} | {} ]", self.domain, self.action).and_then(|x: ()| {
            if let Some(meta_text) = self.meta {
                write!(f, " ({})", meta_text)
            } else {
                Ok(x)
            }
        })
    }
}

impl Error for HyperTaskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl From<HyperTaskError> for String {
    fn from(error: HyperTaskError) -> Self {
        format!("{}", error)
    }
}
