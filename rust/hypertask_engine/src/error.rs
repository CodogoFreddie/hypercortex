use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum HyperTaskErrorDomain {
    Config,
    Context,
    Input,
    Mutation,
    Query,
    ScoreCalculator,
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
                HyperTaskErrorDomain::Input => "input",
                HyperTaskErrorDomain::Mutation => "mutation",
                HyperTaskErrorDomain::Query => "query",
                HyperTaskErrorDomain::ScoreCalculator => "scoreCalculator",
                HyperTaskErrorDomain::Task => "task",
            }
        )
    }
}

#[derive(Debug)]
pub enum HyperTaskErrorAction {
    Create,
    Delete,
    Parse,
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
                HyperTaskErrorAction::Parse => "parse",
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
    meta: Option<String>,
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
        self.meta = Some(meta.to_owned());
        self
    }

    pub fn with_msg<F: Fn() -> String>(mut self, meta_factory: F) -> Self {
        self.meta = Some(meta_factory());
        self
    }

    pub fn from<E: 'static + Error>(mut self, source: E) -> Self {
        self.source = Some(Box::new(source));
        self
    }
}

impl fmt::Display for HyperTaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HyperTaskError[{}::{}]", self.domain, self.action)?;

        if let Some(meta_text) = &self.meta {
            write!(f, ": {}", meta_text)?
        }

        Ok(())
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

pub fn print_error_chain(err: &(dyn Error + 'static)) -> () {
    print_error_chain_recursive(err, 1)
}

pub fn print_error_chain_recursive(err: &(dyn Error + 'static), i: u32) -> () {
    println!("Error {}: {}", i, err);

    if let Some(boxed_source) = err.source() {
        print_error_chain_recursive(boxed_source, i + 1);
    }
}
