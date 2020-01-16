extern crate hypertask_client_cli;
extern crate hypertask_engine;

use hypertask_client_cli::config::CliConfig;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::get_input_tasks;
use std::collections::HashMap;
use std::rc::Rc;

fn complete_id(tasks: &HashMap<Rc<Id>, Rc<Task>>, partial: &str) -> Vec<String> {
    let mut output = vec![];
    for id in tasks.keys() {
        if id.get_string().starts_with(partial) {
            output.push(id.get_string().clone())
        }
    }

    return output;
}

fn main() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("client.toml")?;
    let config_file_getter: ConfigFileGetter<CliConfig> = config_file_opener.parse()?;
    let cli_config: &CliConfig = config_file_getter.get_config();

    let tasks: HashMap<Rc<Id>, Rc<Task>> = get_input_tasks(&*cli_config)?;

    let input_args: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = input_args[1].split(" ").collect();
    let command_being_completed: usize = input_args[2].parse().unwrap();

    let arg_being_completed = args[command_being_completed - 1];

    let id_completions = complete_id(&tasks, &arg_being_completed);

    println!("{}", id_completions.join(" "));

    Ok(())
}
