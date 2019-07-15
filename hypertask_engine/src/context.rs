use crate::task::Task;
use chrono::prelude::*;

pub trait Context {
    fn get_now(&self) -> DateTime<Utc>;
    fn get_input_tasks_iter(
        &self,
    ) -> Result<Box<dyn Iterator<Item = Result<Task, String>>>, String>;
    fn put_task(&self, task: &Task) -> Result<(), String>;
}
