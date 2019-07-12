use crate::engine::{Mutation, Mutations, Queries, Query};
use crate::id::Id;
use crate::prop::Prop;
use crate::recur::Recur;
use crate::tag::{Sign, Tag};
use chrono::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    created_at: DateTime<Utc>,
    description: Option<String>,
    done: Option<DateTime<Utc>>,
    due: Option<DateTime<Utc>>,
    id: Id,
    recur: Option<Recur>,
    snooze: Option<DateTime<Utc>>,
    updated_at: DateTime<Utc>,
    wait: Option<DateTime<Utc>>,

    #[serde(serialize_with = "ordered_set")]
    tags: HashSet<String>,
}

fn ordered_set<S>(value: &HashSet<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut vec = value.iter().collect::<Vec<&String>>();

    vec.sort();

    vec.serialize(serializer)
}

impl Task {
    pub fn generate() -> Self {
        Self {
            created_at: Utc::now(),
            description: None,
            done: None,
            due: None,
            id: Id::generate(),
            recur: None,
            snooze: None,
            tags: HashSet::new(),
            updated_at: Utc::now(),
            wait: None,
        }
    }

    pub fn get_created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
    pub fn get_description(&self) -> &Option<String> {
        &self.description
    }
    pub fn get_done(&self) -> &Option<DateTime<Utc>> {
        &self.done
    }
    pub fn get_due(&self) -> &Option<DateTime<Utc>> {
        &self.due
    }
    pub fn get_id(&self) -> &Id {
        &self.id
    }
    pub fn get_recur(&self) -> &Option<Recur> {
        &self.recur
    }
    pub fn get_snooze(&self) -> &Option<DateTime<Utc>> {
        &self.snooze
    }
    pub fn get_tags(&self) -> &HashSet<String> {
        &self.tags
    }
    pub fn get_updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }
    pub fn get_wait(&self) -> &Option<DateTime<Utc>> {
        &self.wait
    }

    pub fn satisfies_queries(&self, queries: &Queries) -> bool {
        if queries.len() == 0 {
            return false;
        }

        let mut default = false;

        for query in queries {
            match query {
                Query::Id(id) => {
                    if id == &self.id {
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

        return default;
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
                if let Some(recur) = &self.recur {
                    let dt: Duration = Duration::from(recur);

                    if let Some(due) = self.due {
                        self.due = Some(due + dt);
                    }
                    if let Some(wait) = self.wait {
                        self.wait = Some(wait + dt);
                    }
                } else {
                    self.done = Some(done.clone());
                }
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
            Mutation::SetProp(Prop::Recur(recur)) => self.recur = recur.clone(),
        }

        self.updated_at = Utc::now();

        self
    }

    pub fn finalise(self) -> FinalisedTask {
        FinalisedTask {
            score: self.get_score(),
            task: self,
        }
    }

    fn get_score(&self) -> u64 {
        //this is perfectly fine for now, but I'd like to aim to replace this with
        //something user-configureable, possibly https://github.com/jonathandturner/rhai

        let mut score: u64 = 0;

        if let Some(_) = self.done {
            return 0;
        }

        if let Some(wait) = self.wait {
            if wait > Utc::now() {
                return 0;
            }
        }

        if let Some(snooze) = self.snooze {
            if snooze > Utc::now() {
                return 0;
            }
        }

        score = score + (Utc::now() - self.updated_at).num_seconds() as u64;

        if let Some(due) = self.due {
            score = score
                + if self.tags.contains("timely") && due < Utc::now() {
                    2 * (2147483647 - (due.timestamp() as u64))
                } else {
                    (2147483647 - (due.timestamp() as u64))
                };
        }

        score = score
            + if self.tags.contains("urgent") {
                score
            } else {
                0
            };

        score
    }

    pub fn is_overdue(&self) -> bool {
        if let Some(due) = self.due {
            return due < Utc::now();
        } else {
            return false;
        }
    }

    pub fn is_soon_due(&self) -> bool {
        if let Some(due) = self.due {
            return due < (Utc::now() + Duration::days(3));
        } else {
            return false;
        }
    }
}

#[derive(Debug)]
pub struct FinalisedTask {
    task: Task,
    score: u64,
}

impl FinalisedTask {
    pub fn get_task(&self) -> &Task {
        &self.task
    }
    pub fn get_score(&self) -> &u64 {
        &self.score
    }
}

impl PartialOrd for FinalisedTask {
    fn partial_cmp(&self, other: &FinalisedTask) -> Option<Ordering> {
        Some(self.score.cmp(&other.score).reverse())
    }
}

impl Ord for FinalisedTask {
    fn cmp(&self, other: &FinalisedTask) -> Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl Eq for FinalisedTask {}
impl PartialEq for FinalisedTask {
    fn eq(&self, other: &FinalisedTask) -> bool {
        self.score == other.score
    }
}
