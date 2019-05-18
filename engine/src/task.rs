use crate::id::Id;
use crate::interface::{Mutation, Query};
use crate::period::Period;
use crate::tag::Tag;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering, PartialOrd};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    created_at: DateTime<Utc>,
    deleted: bool,
    description: String,
    done: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    id: Id,
    modified_at: DateTime<Utc>,
    recur: Option<Period>,
    snooze: Option<DateTime<Utc>>,
    tags: Vec<Tag>,
    wait: Option<DateTime<Utc>>,

    score: Option<f64>,
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

    pub fn get_id(&self) -> Id {
        self.id.clone()
    }

    pub fn calculate_score(&mut self) -> () {
        self.score = Some(3.0);
    }
}

impl Eq for Task {}
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).unwrap_or(Ordering::Less)
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.score.partial_cmp(&other.score))
    }
}
