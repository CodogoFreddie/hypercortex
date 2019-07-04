use crate::engine::{Mutation, Mutations, Queries, Query};
use crate::id::Id;
use crate::prop::Prop;
use crate::tag::{Sign, Tag};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    created_at: DateTime<Utc>,
    description: Option<String>,
    done: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    id: Id,
    snooze: Option<DateTime<Utc>>,
    tags: HashSet<String>,
    updated_at: DateTime<Utc>,
    wait: Option<DateTime<Utc>>,
}

impl Task {
    pub fn generate() -> Self {
        Self {
            created_at: Utc::now(),
            description: None,
            done: None,
            due: None,
            id: Id::generate(),
            snooze: None,
            tags: HashSet::new(),
            updated_at: Utc::now(),
            wait: None,
        }
    }

    pub fn get_id(&self) -> &Id {
        &(self.id)
    }

    pub fn satisfies_queries(&self, queries: &Queries) -> bool {
        for q in queries {
            if self.satisfies_query(q) {
                continue;
            } else {
                return false;
            }
        }

        true
    }

    pub fn satisfies_query(&self, query: &Query) -> bool {
        match query {
            Query::Id(id) => id == &self.id,
            Query::Tag(Tag { sign, name }) => self.tags.contains(name),
        }
    }

    pub fn apply_mutations(&mut self, mutations: &Mutations) -> &Self {
        for m in mutations {
            self.apply_mutation(m);
        }

        self
    }

    pub fn apply_mutation(&mut self, mutation: &Mutation) -> &Self {
        match mutation {
            Mutation::SetTag(Tag {
                sign: Sign::Plus,
                name,
            }) => {
                self.tags.insert(name.to_string());
            }
            Mutation::SetTag(Tag {
                sign: Sign::Minus,
                name,
            }) => {
                self.tags.remove(name);
            }
            Mutation::SetProp(Prop::Description(description)) => {
                self.description = Some(description.to_string());
            }
            Mutation::SetProp(Prop::Done(done)) => {
                self.done = Some(done.clone());
            }
            Mutation::SetProp(Prop::Due(due)) => {
                self.due = due.clone();
            }
            Mutation::SetProp(Prop::Snooze(snooze)) => {
                self.snooze = snooze.clone();
            }
            Mutation::SetProp(Prop::Wait(wait)) => {
                self.wait = wait.clone();
            }
        }

        self.updated_at = Utc::now();

        self
    }
}
