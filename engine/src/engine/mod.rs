use super::interface::{Mutation, Query};
use super::task::{Id, Task};
use crate::error::CortexResult;
use std::marker::PhantomData;

pub trait EngineDriver<GotTasks: Iterator<Item = Task>> {
    fn setup(&mut self) -> CortexResult<()>;
    fn mount(&mut self) -> CortexResult<()>;
    fn get_tasks(&self) -> GotTasks;
    fn put_task(&mut self, task: Task) -> CortexResult<Task>;
    fn del_task(&mut self, task: Task) -> CortexResult<Task>;
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

pub struct Engine<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>> {
    driver: Driver,
    mutations: Option<Vec<Mutation>>,
    phantom: PhantomData<Input>,
    queries: Option<Vec<Query>>,
}

impl<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>> Engine<Input, Driver> {
    pub fn new(mut driver: Driver) -> Self {
        Self {
            driver,
            mutations: None,
            phantom: PhantomData,
            queries: None,
        }
    }

    pub fn with_queries(mut self, queries: Vec<Query>) -> Self {
        self.queries = Some(queries);

        self
    }

    pub fn with_mutations(mut self, mutations: Vec<Mutation>) -> Self {
        self.mutations = Some(mutations);

        self
    }

    pub fn iter(mut self) -> CortexResult<EngineIterator<Input, Driver>> {
        self.driver.setup()?;
        self.driver.mount()?;

        Ok(EngineIterator::new(
            self.driver,
            self.queries,
            self.mutations,
        ))
    }
}

pub struct EngineIterator<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>> {
    driver: Driver,
    input: Input,
    mutations: Option<Vec<Mutation>>,
    queries: Option<Vec<Query>>,
}

impl<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>> EngineIterator<Input, Driver> {
    pub fn new(
        driver: Driver,
        queries: Option<Vec<Query>>,
        mutations: Option<Vec<Mutation>>,
    ) -> Self {
        let input = driver.get_tasks();

        Self {
            driver,
            input,
            mutations,
            queries,
        }
    }

    fn is_delete_mutation(&self) -> bool {
        if let Some(the_mutations) = &self.mutations {
            the_mutations.iter().any(|m| match m {
                Mutation::Delete => true,
                _ => false,
            })
        } else {
            false
        }
    }
}

impl<Input: Iterator<Item = Task>, Driver: EngineDriver<Input>> Iterator
    for EngineIterator<Input, Driver>
{
    type Item = CortexResult<Task>;

    fn next(&mut self) -> Option<Self::Item> {
        let task = self.input.next()?;

        match &self.queries {
            None => Some(Ok(task)),
            Some(qs) => {
                if task.satisfies_queries(&qs) {
                    match &self.mutations {
                        None => Some(Ok(task)),
                        Some(ms) => {
                            if self.is_delete_mutation() {
                                Some(self.driver.del_task(task))
                            } else {
                                Some(self.driver.put_task(task.apply_mutations(ms)))
                            }
                        }
                    }
                } else {
                    self.next()
                }
            }
        }
    }
}
