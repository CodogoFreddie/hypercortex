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
    pub mutated_tasks: Vec<Rc<Task>>,
    pub display_tasks: Vec<(bool, Score, Rc<Task>)>,
}

pub struct Engine {
    ///these tasks do not change while the engine runs, and will not be mutated
    all_tasks_collection: HashMap<Rc<Id>, Rc<Task>>,
    #[allow(dead_code)]
    dependants_map: HashMap<Rc<Id>, Vec<Rc<Task>>>,

    filter_machine: StackMachine,
    score_machine: StackMachine,
    now: DateTime<Utc>,
}

impl Engine {
    /// Creates a new `Engine`, and builds a map of dependants for each task.
    pub fn new(
        all_tasks_collection: HashMap<Rc<Id>, Rc<Task>>,
        filter_machine: StackMachine,
        score_machine: StackMachine,
        now: DateTime<Utc>,
    ) -> Self {
        let mut dependants_map: HashMap<Rc<Id>, Vec<Rc<Task>>> = HashMap::new();

        for (_child_id, child_task) in all_tasks_collection.iter() {
            if let Some(parent_id) = child_task.get_blocked_by() {
                dependants_map
                    .entry(parent_id.clone())
                    .and_modify(|children: &mut Vec<Rc<Task>>| {
                        children.push(child_task.clone());
                    })
                    .or_insert_with(|| vec![child_task.clone()]);
            }
        }

        Self {
            all_tasks_collection,
            dependants_map,

            filter_machine,
            score_machine,
            now,
        }
    }

    pub fn run(&mut self, command: Command) -> HyperTaskResult<EngineOutput> {
        let mut mutated_tasks: Vec<Rc<Task>> = vec![];
        let mut display_ids: HashSet<Rc<Id>> = HashSet::new();

        match command {
            //actually perform mutations
            Command::Create(mutations) => {
                let new_task: Rc<Task> =
                    Rc::new(Task::generate(&self.now).apply_mutations(&mutations, &self.now));
                let id = new_task.get_id();

                self.all_tasks_collection
                    .insert(id.clone(), new_task.clone());
                mutated_tasks.push(new_task);
                display_ids.insert(id.clone());
            }
            Command::Update(query, mutation) => {
                for (id, task) in self.all_tasks_collection.iter() {
                    // don't run mutations on tasks that are filtered out, the user probably
                    // didn't mean to
                    if task.satisfies_queries(&query)
                        && self.filter_machine.run_on(&task, &self.dependants_map)? > 0.0
                    {
                        let updated_task: Task = task.apply_mutations(&mutation, &self.now);

                        mutated_tasks.push(Rc::new(updated_task));
                        display_ids.insert(id.clone());
                    }
                }
            }

            //if we're just querying, run the query
            Command::Read(query) => {
                for (id, task) in self.all_tasks_collection.iter() {
                    // if there's any query specified
                    if !query.is_empty() {
                        //then return any tasks that match the query, including filtered ones
                        if task.satisfies_queries(&query) {
                            display_ids.insert(id.clone());
                        };
                    } else {
                        //otherwise, filter out queries that don't satisfy the filter
                        if self.filter_machine.run_on(&task, &self.dependants_map)? > 0.0 {
                            display_ids.insert(id.clone());
                        }
                    }
                }
            }
            _ => {}
        };

        for task in &mutated_tasks {
            self.all_tasks_collection
                .insert(task.get_id(), task.clone());
        }

        let mut display_tasks: Vec<(bool, Score, Rc<Task>)> = Vec::with_capacity(display_ids.len());

        for id in display_ids.into_iter() {
            let task: Rc<Task> = self
                .all_tasks_collection
                .get(&id)
                .expect("if I have the Id, I should have the Task")
                .clone();

            let score = self.score_machine.run_on(&task, &self.dependants_map)?;
            let filter = self.filter_machine.run_on(&task, &self.dependants_map)?;

            display_tasks.push((filter > 0.0, score, task));
        }

        display_tasks.sort_unstable_by(|(_, a, _), (_, b, _)| b.partial_cmp(a).unwrap());

        Ok(EngineOutput {
            mutated_tasks,
            display_tasks,
        })
    }
}
