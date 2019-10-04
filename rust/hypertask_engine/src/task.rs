use crate::engine::{HyperTaskEngineContext, Mutation, Query};
use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::recur::Recur;
use crate::tag::{Sign, Tag};
use chrono::prelude::*;
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::HashSet;
use time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    done: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    due: Option<DateTime<Utc>>,
    id: Id,
    #[serde(skip_serializing_if = "Option::is_none")]
    recur: Option<Recur>,
    #[serde(skip_serializing_if = "Option::is_none")]
    snooze: Option<DateTime<Utc>>,
    updated_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    wait: Option<DateTime<Utc>>,

    #[serde(serialize_with = "ordered_set")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
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
    pub fn generate<
        InputIterator: Iterator<Item = HyperTaskResult<Task>>,
        Context: HyperTaskEngineContext<InputIterator>,
    >(
        context: &mut Context,
    ) -> Self {
        Self {
            created_at: context.get_now(),
            description: None,
            done: None,
            due: None,
            id: Id::generate(context),
            recur: None,
            snooze: None,
            tags: HashSet::new(),
            updated_at: context.get_now(),
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

    pub fn satisfies_queries(&self, queries: &[Query]) -> bool {
        if queries.is_empty() {
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

    pub fn finalise(self, stack_machine: &mut StackMachine) -> HyperTaskResult<FinalisedTask> {
        Ok(FinalisedTask {
            score: self.get_score(stack_machine)?,
            task: self,
        })
    }

    fn get_score(&self, stack_machine: &mut StackMachine) -> HyperTaskResult<f64> {
        stack_machine.run_on(self)
    }

    pub fn is_overdue(&self, now: &DateTime<Utc>) -> bool {
        if let Some(due) = self.due {
            return due < *now;
        } else {
            return false;
        }
    }

    pub fn is_soon_due(&self, now: &DateTime<Utc>) -> bool {
        if let Some(due) = self.due {
            return due < (*now + Duration::days(3));
        } else {
            return false;
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FinalisedTask {
    task: Task,
    score: f64,
}

impl FinalisedTask {
    pub fn get_task(&self) -> &Task {
        &self.task
    }
    pub fn get_score(&self) -> &f64 {
        &self.score
    }
}

impl PartialOrd for FinalisedTask {
    fn partial_cmp(&self, other: &FinalisedTask) -> Option<Ordering> {
        self.score.partial_cmp(&other.score).map(|x| x.reverse())
    }
}

impl Ord for FinalisedTask {
    fn cmp(&self, other: &FinalisedTask) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

impl Eq for FinalisedTask {}
impl PartialEq for FinalisedTask {
    fn eq(&self, other: &FinalisedTask) -> bool {
        self.score == other.score
    }
}
