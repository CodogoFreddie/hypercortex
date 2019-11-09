use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::rpn::StackMachine;
use crate::tag::Tag;
use crate::task::{FinalisedTask, Task};
use chrono::prelude::*;
use std::collections::HashMap;

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

pub trait HyperTaskEngineContext<TaskIterator: Iterator<Item = HyperTaskResult<Task>>> {
    fn finalize_mutations(&self) -> HyperTaskResult<()>;
    fn generate_id(&mut self) -> String;
    fn get_now(&self) -> DateTime<Utc>;
    fn get_score_machine(&self) -> HyperTaskResult<StackMachine>;
    fn get_filter_machine(&self) -> HyperTaskResult<StackMachine>;
    fn get_task_iterator(&self) -> HyperTaskResult<TaskIterator>;
    fn put_task(&mut self, task: &Task) -> HyperTaskResult<()>;
}

pub fn run<InputIterator, Context>(
    command: Command,
    context: Context,
) -> HyperTaskResult<Vec<FinalisedTask>>
where
    InputIterator: Iterator<Item = HyperTaskResult<Task>>,
    Context: HyperTaskEngineContext<InputIterator>,
{
    let mut score_machine = context.get_score_machine()?;
    let mut filter_machine = context.get_filter_machine()?;

    let input_iterator = context.get_task_iterator()?;

    let tasks_map: HashMap<Id, Task> = input_iterator
        .map(|task_result| task_result.map(|task| (task.get_id().clone(), task)))
        .collect::<HyperTaskResult<HashMap<Id, Task>>>()?;

    dbg!(&tasks_map);

    Ok(vec![])
}
