extern crate hypercortex_engine;

use hypercortex_engine::*;

struct MockDriver {
    tasks: Vec<Task>,
}

impl EngineDriver<std::slice::Iter<'static, Task>> for MockDriver {
    fn setup(&mut self) -> CortexResult<()> {
        Ok(())
    }
    fn mount(&mut self) -> CortexResult<()> {
        Ok(())
    }
    fn get_tasks(&self) -> std::slice::Iter<'static, Task> {
        self.tasks.iter().to_owned()
    }
    fn put_task(&mut self, task: Task) -> CortexResult<Task> {
        println!("put {}", task);

        task
    }
    fn del_task(&mut self, task: Task) -> CortexResult<Task> {
        println!("del {}", task);

        task
    }
}

#[test]
fn can_make_new_task() {}
