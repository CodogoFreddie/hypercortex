use super::interface::{Mutation, Query};
use super::task::{Id, Task};
use crate::error::CortexResult;
use std::marker::PhantomData;

pub fn run<Persister: Fn(Task) -> CortexResult<Task>, Input: Iterator<Item = Task>>(
    persister: Persister,
    queries: Option<Vec<Query>>,
    mutations: Option<Vec<Mutation>>,
    input: Input,
) -> Vec<CortexResult<Task>> {
    let mut all_ids = Vec::new();

    input
        .filter_map(|task| {
            // make a collection of all Ids, so we can find the uniqely identifiable prefix of the
            // selected Tasks
            all_ids.push(task.get_id().clone());

            // return only the Tasks that match the query
            match &queries {
                None => Some(task),
                Some(qs) => {
                    if task.satisfies_queries(qs) {
                        Some(task)
                    } else {
                        None
                    }
                }
            }
        })
        .map(
            // apply the mutations to each task that's been selected
            |task| match &mutations {
            None => Ok(task),
            // if there are mutations to apply, persist the task once they've been applied
            Some(ms) => persister(task.apply_mutations(ms)),
        })
        .collect()
}
