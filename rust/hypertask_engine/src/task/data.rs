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

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub(super) created_at: DateTime<Utc>,
    pub(super) updated_at: DateTime<Utc>,
    pub(super) id: Id,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) depends_on: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) done: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) due: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) recur: Option<Recur>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) snooze: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) wait: Option<DateTime<Utc>>,

    #[serde(serialize_with = "ordered_set")]
    #[serde(skip_serializing_if = "HashSet::is_empty")]
    #[serde(default)]
    pub(super) tags: HashSet<String>,
}

fn ordered_set<S>(value: &HashSet<String>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut vec = value.iter().collect::<Vec<&String>>();

    vec.sort();

    vec.serialize(serializer)
}

impl Hash for Task {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.created_at.hash(state);
        self.depends_on.hash(state);
        self.description.hash(state);
        self.done.hash(state);
        self.due.hash(state);
        self.id.hash(state);
        self.recur.hash(state);
        self.snooze.hash(state);
        self.updated_at.hash(state);
        self.wait.hash(state);

        let mut tags_vec: Vec<&String> = self.tags.iter().collect();
        tags_vec.sort();
        tags_vec.hash(state);
    }
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
            depends_on: None,
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
}
