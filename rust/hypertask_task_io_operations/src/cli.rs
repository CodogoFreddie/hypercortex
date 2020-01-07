use super::*;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufWriter, ErrorKind};

pub fn delete_task<Config: ProvidesDataDir>(config: &Config, id: &Id) -> HyperTaskResult<()> {
    let task_state_dir: &PathBuf = config.get_task_state_dir();

    let Id(task_id) = id;
    let file_path = task_state_dir.join(task_id);

    match fs::remove_file(file_path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(HyperTaskError::new(
            HyperTaskErrorDomain::Task,
            HyperTaskErrorAction::Delete,
        )
        .with_msg(|| format!("could not delete file for task with id `{}`", task_id))
        .from(e)),
    }
}

pub fn put_task<Config: ProvidesDataDir>(config: &Config, task: &Task) -> HyperTaskResult<()> {
    let task_state_dir: &PathBuf = config.get_task_state_dir();

    let Id(task_id) = &*task.get_id();

    let file_path = task_state_dir.join(task_id);

    let file = File::create(file_path).map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
            .with_msg(|| {
                format!(
                    "could not create file handle for task with id `{}`",
                    task_id
                )
            })
            .from(e)
    })?;
    let buf_writer = BufWriter::new(file);

    serde_json::to_writer_pretty(buf_writer, &task).map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
            .with_msg(|| format!("could not serialize task with id `{}`", task_id))
            .from(e)
    })?;

    Ok(())
}

pub fn get_task<Config: ProvidesDataDir>(
    config: &Config,
    id: &Id,
) -> HyperTaskResult<Option<Task>> {
    let task_state_dir: &PathBuf = config.get_task_state_dir();

    let task_file_path = task_state_dir.join(id.0.clone());

    let task_file = match File::open(&task_file_path) {
        Ok(t) => t,
        Err(e) => {
            if e.kind() == ErrorKind::NotFound {
                return Ok(None);
            } else {
                return Err(HyperTaskError::new(
                    HyperTaskErrorDomain::Task,
                    HyperTaskErrorAction::Read,
                )
                .with_msg(|| format!("failed to open task `{:?}`", task_file_path))
                .from(e));
            }
        }
    };

    let task: Task = serde_json::from_reader::<std::io::BufReader<std::fs::File>, Task>(
        BufReader::new(task_file),
    )
    .map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
            .with_msg(|| format!("failed to parse task @ `{:?}`", task_file_path))
            .from(e)
    })?;

    Ok(Some(task))
}

pub fn get_input_tasks<Config: ProvidesDataDir>(
    config: &Config,
) -> HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>> {
    let task_state_dir: &PathBuf = config.get_task_state_dir();
    let task_files_iterator = fs::read_dir(&task_state_dir).map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
            .with_msg(|| {
                format!(
                    "folder `{:}` could not be found",
                    &task_state_dir.to_str().unwrap_or("")
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
