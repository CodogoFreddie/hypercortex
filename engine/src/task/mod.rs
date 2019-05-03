pub(crate) mod id;
pub(crate) mod period;
pub(crate) mod tag;

use super::interface::{Mutation, Query};
use chrono::prelude::*;
pub use id::Id;
pub use period::Period;
pub use tag::Tag;

pub struct Task {
    created_at: DateTime<Utc>,
    description: String,
    done: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    id: Id,
    modified_at: DateTime<Utc>,
    recur: Option<Period>,
    snooze: Option<DateTime<Utc>>,
    tags: Vec<Tag>,
    wait: Option<DateTime<Utc>>,
}

impl Task {
    pub fn satisfies_query(&self, query: &Query) -> bool {
        true
    }

    pub fn satisfies_queries(&self, queries: &[Query]) -> bool {
        queries.iter().all(|q| self.satisfies_query(q))
    }

    pub fn apply_mutation(mut self, mutation: &Mutation) -> Self {
        self
    }

    pub fn apply_mutations(mut self, mutations: &[Mutation]) -> Self {
        mutations
            .iter()
            .fold(self, |task, mutation| task.apply_mutation(mutation))
    }
}
