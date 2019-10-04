use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use crate::task::{FinalisedTask, Task};
use chrono::prelude::*;
use std::collections::{HashMap, HashSet};

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

pub trait GetNow {
    fn get_now(&self) -> DateTime<Utc>;
}

pub trait PutTask {
    fn put_task(&mut self, task: &Task) -> HyperTaskResult<()>;
}

pub trait FinalizeMutations {
    fn finalize_mutations(&self) -> HyperTaskResult<()>;
}

pub trait GenerateId {
    fn generate_id(&mut self) -> String;
}

pub trait GetTaskIterator<TaskIterator: Iterator<Item = HyperTaskResult<Task>>> {
    fn get_task_iterator(&self) -> HyperTaskResult<TaskIterator>;
}

struct TaskEngine<
    InputIterator: Iterator<Item = HyperTaskResult<Task>>,
    Context: GetNow + PutTask + GenerateId + GetTaskIterator<InputIterator> + FinalizeMutations,
> {
    command: Command,
    input_iterator: InputIterator,
    context: Context,
    now: DateTime<Utc>,
    done: bool,
}

impl<
        InputIterator: Iterator<Item = HyperTaskResult<Task>>,
        Context: GetNow + PutTask + GenerateId + GetTaskIterator<InputIterator> + FinalizeMutations,
    > TaskEngine<InputIterator, Context>
{
    pub fn new(command: Command, context: Context) -> HyperTaskResult<Self> {
        let now = context.get_now();
        let input_iterator = context.get_task_iterator()?;

        Ok(Self {
            command,
            input_iterator,
            context,
            now: Utc::now(),
            done: false,
        })
    }

    fn yield_next_task(&mut self) -> Option<HyperTaskResult<Task>> {
        if (self.done) {
            return None;
        }

        match &self.command {
            Command::Read(queries) => match self.input_iterator.next()? {
                Err(e) => Some(Err(e)),
                Ok(next_task) => {
                    if (queries.len() == 0 || next_task.satisfies_queries(&queries)) {
                        Some(Ok(next_task))
                    } else {
                        self.next()
                    }
                }
            },

            Command::Create(mutations) => {
                let mut new_task = Task::generate(&mut self.context);
                new_task.apply_mutations(mutations, &self.now);

                self.done = true;

                Some(self.context.put_task(&new_task).map(|_| new_task))
            }

            Command::Update(queries, mutations) => match self.input_iterator.next()? {
                Err(e) => Some(Err(e)),
                Ok(mut next_task) => {
                    if (next_task.satisfies_queries(&queries)) {
                        next_task.apply_mutations(mutations, &self.now);

                        Some(self.context.put_task(&next_task).map(|_| next_task))
                    } else {
                        self.next()
                    }
                }
            },

            Command::Delete(query) => {
                panic!("fuck you");
                None
            }
        }
    }
}

impl<
        InputIterator: Iterator<Item = HyperTaskResult<Task>>,
        Context: GetNow + PutTask + GenerateId + GetTaskIterator<InputIterator> + FinalizeMutations,
    > Iterator for TaskEngine<InputIterator, Context>
{
    type Item = HyperTaskResult<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.yield_next_task() {
            Some(task) => Some(task),
            None => {
                match self.command {
                    Command::Read(_) => {}
                    _ => {
                        self.context.finalize_mutations();
                    }
                };
                None
            }
        }
    }
}

pub fn run<InputIterator, Context>(
    command: Command,
    mut context: Context,
) -> HyperTaskResult<Vec<FinalisedTask>>
where
    InputIterator: Iterator<Item = HyperTaskResult<Task>>,
    Context: GetNow + PutTask + GenerateId + GetTaskIterator<InputIterator> + FinalizeMutations,
{
    let now = context.get_now();
    let mut task_collection = TaskEngine::new(command, context)?
        .map(|task_result| task_result.map(|task| task.finalise(&now)))
        .collect::<HyperTaskResult<Vec<FinalisedTask>>>()?;

    task_collection.sort_unstable();

    Ok(task_collection)
}
