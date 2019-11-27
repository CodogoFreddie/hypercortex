use crate::config::SyncServerConfig;
use hypertask_engine::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;

pub fn get_input_tasks(config: &SyncServerConfig) -> HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>> {
    let task_files_iterator = fs::read_dir(&config.data_dir).map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
            .with_msg(|| {
                format!(
                    "folder `{:}` could not be found",
                    &config.data_dir.to_str().unwrap_or("")
                )
            })
            .from(e)
    })?;

    let mut map: HashMap<Rc<Id>, Rc<Task>> = HashMap::new();

    for task_file_path_result in task_files_iterator {
        let task_file_path = task_file_path_result.map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("could not open task path for reading")
                .from(e)
        })?;

        let task_file = File::open(task_file_path.path()).map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .with_msg(|| format!("failed to open task `{:?}`", task_file_path))
                .from(e)
        })?;

        let task: Task = serde_json::from_reader::<std::io::BufReader<std::fs::File>, Task>(
            BufReader::new(task_file),
        )
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .with_msg(|| format!("failed to parse task @ `{:?}`", task_file_path))
                .from(e)
        })?;

        map.insert(task.get_id(), Rc::new(task));
    }

    Ok(map)
}
