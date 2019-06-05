use crate::id::Id;
use crate::prop::Prop;
use crate::tag::Tag;
use crate::task::Task;

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

type Mutations = Vec<Mutation>;
type Queries = Vec<Query>;

pub enum CortexEngine {
    Create(Mutations),
    Read(Queries),
    Update(Queries, Mutations),
    Delete(Queries),
}

impl CortexEngine {
    pub fn run(input_tasks_iter: impl Iterator<Item = Task>) -> Vec<Task> {
        vec![]
    }
}
