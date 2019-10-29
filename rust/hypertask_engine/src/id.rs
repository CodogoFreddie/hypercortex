use crate::engine::HyperTaskEngineContext;
use crate::error::*;
use crate::task::Task;
use serde::{Deserialize, Serialize};

use std::fmt;

pub const VALID_ID_CHARS: &str = "23456789abcdefghkmnpqrstwxyz";
pub const NUMBER_OF_CHARS_IN_FULL_ID: usize = 16;

#[derive(Debug, Clone, Serialize, Deserialize, Hash)]
pub struct Id(pub String);

impl Id {
    pub fn generate<
        InputIterator: Iterator<Item = HyperTaskResult<Task>>,
        Context: HyperTaskEngineContext<InputIterator>,
    >(
        context: &mut Context,
    ) -> Self {
        Self(context.generate_id())
    }

    pub fn sub_eq(&self, other: &Id) -> bool {
        let Id(self_content) = self;
        let Id(other_content) = other;

        self_content.contains(other_content.as_str())
            || other_content.contains(self_content.as_str())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let Id(x) = self;
        write!(f, "{}", x)
    }
}
