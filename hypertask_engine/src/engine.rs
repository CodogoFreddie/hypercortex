use crate::context::Context;
use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use crate::task::{FinalisedTask, Task};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum Mutation {
    SetProp(Prop),
    SetTag(Tag),
}

#[derive(Debug)]
pub enum Query {
    Id(Id),
    Tag(Tag),
}

pub type Mutations = Vec<Mutation>;
pub type Queries = Vec<Query>;

pub enum Command {
    Create(Mutations),
    Read(Queries),
    Update(Queries, Mutations),
    Delete(Queries),
}

pub fn run(command: Command, context: &Context) -> Vec<FinalisedTask> {
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
            .map(|r| r.unwrap())
            .filter(|t| queries.len() == 0 || t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .filter(|ft| ft.get_score() != &0)
            .collect::<Vec<FinalisedTask>>(),

        Command::Update(queries, mutations) => context
            .get_input_tasks_iter()
            .map(|r| r.unwrap())
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
            .map(|r| r.unwrap())
            .filter(|t| t.satisfies_queries(queries))
            .map(|t| t.finalise(&now))
            .collect::<Vec<FinalisedTask>>(),
    };

    tasks_collection.sort_unstable();

    tasks_collection
}
