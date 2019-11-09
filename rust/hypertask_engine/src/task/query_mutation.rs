use super::Task;
use crate::engine::{HyperTaskEngineContext, Mutation, Query};
use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::recur::Recur;
use crate::rpn::StackMachine;
use crate::tag::{Sign, Tag};
use chrono::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use time::Duration;

impl super::Task {
    pub fn satisfies_queries(&self, queries: &[Query]) -> bool {
        if queries.is_empty() {
            return false;
        }

        let mut default = false;

        for query in queries {
            match query {
                Query::Id(id) => {
                    if id.sub_eq(&self.id) {
                        return true;
                    } else {
                        continue;
                    }
                }

                Query::Tag(Tag {
                    sign: Sign::Plus,
                    name,
                }) => {
                    if self.tags.contains(name) {
                        return true;
                    } else {
                        continue;
                    }
                }

                Query::Tag(Tag {
                    sign: Sign::Minus,
                    name,
                }) => {
                    if self.tags.contains(name) {
                        return false;
                    } else {
                        default = true;
                        continue;
                    }
                }
            }
        }

        default
    }

    pub fn apply_mutations(&mut self, mutations: &[Mutation], now: &DateTime<Utc>) -> &Self {
        for m in mutations {
            self.apply_mutation(m, now);
        }

        self
    }

    pub fn apply_mutation(&mut self, mutation: &Mutation, now: &DateTime<Utc>) -> &Self {
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
                if let Some(recur) = &self.recur {
                    let dt: Duration = Duration::from(recur);

                    if let Some(due) = self.due {
                        self.due = Some(due + dt);
                    }
                    if let Some(wait) = self.wait {
                        self.wait = Some(wait + dt);
                    }
                } else {
                    self.done = Some(*done);
                }
            }
            Mutation::SetProp(Prop::Due(due)) => {
                self.due = *due;
            }
            Mutation::SetProp(Prop::Snooze(snooze)) => {
                self.snooze = *snooze;
            }
            Mutation::SetProp(Prop::Wait(wait)) => {
                self.wait = *wait;
            }
            Mutation::SetProp(Prop::Recur(recur)) => self.recur = recur.clone(),
        }

        self.updated_at = *now;

        self
    }
}
