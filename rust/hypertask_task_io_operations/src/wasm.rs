use super::*;

fn get_local_storage() -> HyperTaskResult<web_sys::Storage> {
    let window = web_sys::window().ok_or(
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
            .msg("can't get window"),
    )?;

    window
        .local_storage()
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("can't get local_storage")
        })?
        .ok_or(
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("can't get local_storage"),
        )
}

pub fn delete_task<Config>(_: &Config, id: &Id) -> HyperTaskResult<()> {
    let local_storage = get_local_storage()?;

    local_storage
        .delete(&format!("hypertask::task::{}", id))
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
                .msg("can't delete task")
        })?;

    Ok(())
}

pub fn put_task<Config>(_: &Config, task: &Task) -> HyperTaskResult<()> {
    let local_storage = get_local_storage()?;

    let serial_task = serde_json::to_string(task).map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
            .msg("can't serialise task")
            .from(e)
    })?;

    local_storage
        .set(&format!("hypertask::task::{}", task.get_id()), &serial_task)
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Write)
                .msg("can't write task")
        })?;

    Ok(())
}

pub fn get_task<Config>(_: &Config, id: &Id) -> HyperTaskResult<Option<Task>> {
    let local_storage = get_local_storage()?;

    let serial_task = local_storage
        .get(&format!("hypertask::task::{}", id))
        .map_err(|_| {
            HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                .msg("can't get task")
        })?;

    match serial_task {
        Some(serial_task) => {
            let task: Task = serde_json::from_str(&serial_task).map_err(|e| {
                HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    .with_msg(|| format!("can't deserialise task `{}`", &id))
                    .from(e)
            })?;

            Ok(Some(task))
        }
        None => Ok(None),
    }
}

pub fn get_input_tasks<Config>(config: &Config) -> HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>> {
    let local_storage = get_local_storage()?;

    let mut tasks = HashMap::new();

    for i in 0..local_storage.length().map_err(|_| {
        HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
            .msg("can't get local storage key")
    })? {
        let key = local_storage
            .key(i)
            .map_err(|_| {
                HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    .msg("can't get local storage key")
            })?
            .ok_or(
                HyperTaskError::new(HyperTaskErrorDomain::Task, HyperTaskErrorAction::Read)
                    .msg("can't get local storage key"),
            )?;

        if key.starts_with("hypertask::task::") {
            if let Some(task) = get_task(config, &Id(key.replace("hypertask::task::", "")))? {
                let contained_task = Rc::new(task);

                tasks.insert(contained_task.get_id().clone(), contained_task);
            }
        }
    }

    Ok(tasks)
}
