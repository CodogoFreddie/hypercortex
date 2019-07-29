use crate::task::Task;
use chrono::prelude::*;

pub trait Context {
    type TaskIterator: Iterator<Item = Result<Task, String>>;

    fn get_now(&self) -> DateTime<Utc>;
    fn get_input_tasks_iter(&self) -> Self::TaskIterator;
    fn put_task(&self, task: &Task) -> Result<(), String>;
}
