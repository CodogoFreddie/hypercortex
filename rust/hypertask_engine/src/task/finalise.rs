pub use super::Task;
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
use time::Duration;

pub enum MaybeFinalisedTask {
    FinalisedTask(FinalisedTask),
    Task(Task),
}

impl Task {
    pub fn finalise(
        self,
        stack_machine: &mut StackMachine,
        filter_machine: &mut StackMachine,
    ) -> HyperTaskResult<Option<FinalisedTask>> {
        if filter_machine.run_on(&self)? > 0.0 {
            Ok(Some(FinalisedTask {
                score: stack_machine.run_on(&self)?,
                task: self,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn is_overdue(&self, now: &DateTime<Utc>) -> bool {
        if let Some(due) = self.due {
            due < *now
        } else {
            false
        }
    }

    pub fn is_soon_due(&self, now: &DateTime<Utc>) -> bool {
        if let Some(due) = self.due {
            due < (*now + Duration::days(3))
        } else {
            false
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
