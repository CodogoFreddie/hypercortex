use crate::context::Context;
use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use crate::task::{FinalisedTask, Task};

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

pub fn run<TaskIterator: Iterator<Item = Result<Task, String>>>(
    command: Command,
    context: &Context<TaskIterator = TaskIterator>,
) -> Result<Vec<FinalisedTask>, String> {
    let now = context.get_now();

    let mut tasks_collection = match &command {
        Command::Create(mutations) => {
            let mut new_task = Task::generate(&now);

            new_task.apply_mutations(mutations, &now);

            context.put_task(&new_task).unwrap();

            vec![new_task.finalise(&now)]
        }

        Command::Read(queries) => context
            .get_input_tasks_iter()
            .map(std::result::Result::unwrap)
            .filter(|t| queries.is_empty() || t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .filter(|ft| ft.get_score() != &0)
            .collect::<Vec<FinalisedTask>>(),

        Command::Update(queries, mutations) => context
            .get_input_tasks_iter()
            .map(std::result::Result::unwrap)
            .filter(|t| t.satisfies_queries(queries))
            .map(|mut t| {
                t.apply_mutations(mutations, &now);
                context.put_task(&t).unwrap();
                t
            })
            .map(|t| t.finalise(&now))
            .collect::<Vec<FinalisedTask>>(),

        Command::Delete(queries) => context
            .get_input_tasks_iter()
            .map(std::result::Result::unwrap)
            .filter(|t| t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .collect::<Vec<FinalisedTask>>(),
    };

    tasks_collection.sort_unstable();

    Ok(tasks_collection)
}
