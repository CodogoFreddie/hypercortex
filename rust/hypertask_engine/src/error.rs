use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HyperTaskErrorDomain {
    Config,
    Context,
    Mutation,
    Query,
    Task,
}

impl fmt::Display for HyperTaskErrorDomain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HyperTaskErrorDomain::Config => "config",
                HyperTaskErrorDomain::Context => "context",
                HyperTaskErrorDomain::Mutation => "mutation",
                HyperTaskErrorDomain::Query => "query",
                HyperTaskErrorDomain::Task => "task",
            }
        )
    }
}

#[derive(Debug)]
pub enum HyperTaskErrorAction {
    Create,
    Delete,
    Read,
    Run,
    Write,
}

impl fmt::Display for HyperTaskErrorAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HyperTaskErrorAction::Create => "create",
                HyperTaskErrorAction::Delete => "delete",
                HyperTaskErrorAction::Read => "read",
                HyperTaskErrorAction::Run => "run",
                HyperTaskErrorAction::Write => "write",
            }
        )
    }
}

#[derive(Debug)]
pub struct HyperTaskError {
    domain: HyperTaskErrorDomain,
    action: HyperTaskErrorAction,
    meta: Option<&'static str>,
    source: Option<Box<dyn Error + 'static>>,
}

pub type HyperTaskResult<T> = Result<T, HyperTaskError>;

impl HyperTaskError {
    pub fn new(domain: HyperTaskErrorDomain, action: HyperTaskErrorAction) -> Self {
        Self {
            domain,
            action,
            meta: None,
            source: None,
        }
    }

    pub fn msg(mut self, meta: &'static str) -> Self {
        self.meta = Some(meta);
        self
    }

    pub fn from<E: 'static + Error>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for HyperTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HyperTaskError [{}::{}]", self.domain, self.action)?;

        if let Some(meta_text) = self.meta {
            write!(f, " ({})", meta_text)
        } else {
            Ok(())
        }
    }
}

impl Error for HyperTaskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        if let Some(source_box) = &self.source {
            Some(source_box.as_ref())
        } else {
            None
        }
    }
}

impl From<HyperTaskError> for String {
    fn from(error: HyperTaskError) -> Self {
        format!("{}", error)
    }
}
