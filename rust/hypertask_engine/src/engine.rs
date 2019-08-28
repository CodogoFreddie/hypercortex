use crate::error::*;
use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use crate::task::{FinalisedTask, Task};
use chrono::prelude::*;

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

pub trait GenerateId {
    fn generate_id(&mut self) -> String;
}

pub trait GetTaskIterator {
    type TaskIterator: Iterator<Item = Result<Task, String>>;

    fn get_task_iterator(&mut self) -> HyperTaskResult<Self::TaskIterator>;
}

//TODO needs a new trait that outputs an owned TaskIterator

pub fn run<Context>(command: Command, mut context: Context) -> HyperTaskResult<Vec<FinalisedTask>>
where
    Context: GetNow + PutTask + GenerateId + GetTaskIterator,
{
    let now = context.get_now();
    let input_iterator = context.get_task_iterator()?;

    let mut tasks_collection = match &command {
        Command::Create(mutations) => {
            let mut new_task = Task::generate(&mut context);

            new_task.apply_mutations(mutations, &now);

            context.put_task(&new_task)?;

            vec![new_task.finalise(&now)]
        }

        Command::Read(queries) => input_iterator
            .map(std::result::Result::unwrap)
            .filter(|t| queries.is_empty() || t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .filter(|ft| ft.get_score() != &0)
            .collect::<Vec<FinalisedTask>>(),

        Command::Update(queries, mutations) => input_iterator
            .map(std::result::Result::unwrap)
            .filter(|t| t.satisfies_queries(queries))
            .map(|mut task| {
                task.apply_mutations(mutations, &now);
                context.put_task(&task)?;
                Ok(task)
            })
            .map(|task_result| task_result.map(|task| task.finalise(&now)))
            .collect::<HyperTaskResult<Vec<FinalisedTask>>>()?,

        Command::Delete(queries) => input_iterator
            .map(std::result::Result::unwrap)
            .filter(|t| t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .collect::<Vec<FinalisedTask>>(),
    };

    tasks_collection.sort_unstable();

    Ok(tasks_collection)
}
