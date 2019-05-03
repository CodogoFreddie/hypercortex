use super::interface::{Mutation, Query};
use super::task::{Id, Task};
use std::marker::PhantomData;

pub trait EngineDriver<GotTasks: Iterator<Item = Task>> {
    fn setup(&mut self) -> Result<(), ()>;
    fn mount(&mut self) -> Result<(), ()>;
    fn get_tasks(&mut self) -> GotTasks;
    fn put_task(&mut self, task: Task) -> Result<Task, ()>;
    fn del_task(&mut self, task: Task) -> Result<Task, ()>;
}

struct EngineOutputIter<I: Iterator<Item = Task>> {
    parent: I,
}

impl<I: Iterator<Item = Task>> EngineOutputIter<I> {
    fn new(parent: I) -> Self {
        Self { parent }
    }
}

impl<I: Iterator<Item = Task>> Iterator for EngineOutputIter<I> {
    type Item = Task;

    fn next(&mut self) -> Option<Self::Item> {
        self.parent.next()
    }
}

pub fn run<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>>(
    mut driver: Driver,
    queries: Vec<Query>,
    mutations: Option<Vec<Mutation>>,
) -> impl Iterator<Item = Result<Task, ()>> {
    driver.setup();
    driver.mount();

    let is_delete_mutation = if let Some(ms) = &mutations {
        ms.iter().any(|m| {
            if let Mutation::Delete = m {
                true
            } else {
                false
            }
        })
    } else {
        false
    };

    driver
        .get_tasks()
        .filter(move |task| task.satisfies_queries(&queries[..]))
        .map(move |task| match &mutations {
            //no mutations, just return
            None => Ok(task),
            //there are mutations
            Some(ms) => {
                if is_delete_mutation {
                    // if any of the mutations are Delete, delete the task
                    driver.del_task(task)
                } else {
                    // otherwise, apply the mutations and save
                    let updated_task = task.apply_mutations(&ms[..]);
                    driver.put_task(updated_task)
                }
            }
        })
}
