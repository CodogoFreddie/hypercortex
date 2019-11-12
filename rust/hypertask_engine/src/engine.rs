use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::rpn::StackMachine;
use crate::tag::Tag;
use crate::task::{Score, Task};
use chrono::prelude::*;
use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Deserialize, Serialize)]
pub enum Mutation {
    SetProp(Prop),
    SetTag(Tag),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Query {
    Id(Id),
    Tag(Tag),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Command {
    Create(Vec<Mutation>),
    Read(Vec<Query>),
    Update(Vec<Query>, Vec<Mutation>),
    Delete(Vec<Query>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EngineOutput {
    mutated_tasks: Vec<Rc<Task>>,
    display_tasks: Vec<(Score, Rc<Task>)>,
}

pub struct Engine {
    tasks: HashMap<Rc<Id>, Rc<Task>>,
    dependants_map: HashMap<Rc<Id>, Vec<Rc<Id>>>,

    command: Command,
    filter_machine: StackMachine,
    score_machine: StackMachine,
    now: DateTime<Utc>,
}

impl Engine {
    pub fn new(
        command: Command,
        tasks: HashMap<Rc<Id>, Rc<Task>>,
        filter_machine: StackMachine,
        score_machine: StackMachine,
        now: DateTime<Utc>,
    ) -> Self {
        let mut dependants_map: HashMap<Rc<Id>, Vec<Rc<Id>>> = HashMap::new();

        for (child_id, task) in tasks.iter() {
            if let Some(parent_id) = task.get_depends_on() {
                dependants_map
                    .entry(parent_id.clone())
                    .and_modify(|children: &mut Vec<Rc<Id>>| {
                        children.push(child_id.clone());
                    })
                    .or_insert_with(|| vec![child_id.clone()]);
            }
        }

        Self {
            tasks,
            dependants_map,

            command,
            filter_machine,
            score_machine,
            now,
        }
    }

    pub fn run(&mut self) -> HyperTaskResult<EngineOutput> {
        let mut mutated_tasks = vec![];
        let mut display_tasks = vec![];

        for (id, task) in self.tasks.iter() {
            dbg!(&(id, task));
        }

        Ok(EngineOutput {
            mutated_tasks,
            display_tasks,
        })
    }
}
