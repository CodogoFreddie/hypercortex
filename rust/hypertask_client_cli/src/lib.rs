#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate chrono_english;
extern crate hypertask_config_file_opener;
extern crate hypertask_engine;
extern crate render_simple_cli_table;
extern crate shellexpand;

mod auto_complete_creator;
pub mod config;
mod parse_args;
mod render;

use crate::auto_complete_creator::create_auto_complete_files;
use crate::config::CliConfig;
use crate::parse_args::parse_cli_args;
use crate::render::render_engine_output;
use chrono::prelude::*;
use hypertask_config_file_opener::run_string_as_shell_command;
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::{get_input_tasks, put_task};
use std::collections::HashMap;
use std::rc::Rc;

fn create_stack_machine(now: &DateTime<Utc>, program: Vec<RPNSymbol>) -> StackMachine {
    let mut env = HashMap::new();

    env.insert("day_of_week", f64::from(now.weekday().number_from_monday()));
    env.insert("hour", f64::from(now.hour()));
    env.insert("minute", f64::from(now.minute()));
    env.insert("month", f64::from(now.month()));
    env.insert("now", now.timestamp() as f64);

    StackMachine::new(program, env)
}

pub fn run_cli(args: &[String]) -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("client.toml")?;
    let config_file_getter: ConfigFileGetter<CliConfig> = config_file_opener.parse()?;
    let cli_config: &CliConfig = config_file_getter.get_config();

    if create_auto_complete_files() {
        return Ok(());
    }

    let tasks: HashMap<Rc<Id>, Rc<Task>> = get_input_tasks(&*cli_config)?;
    let now = Utc::now();
    let score_machine = create_stack_machine(&now, cli_config.score_calculator.to_program());
    let filter_machine = create_stack_machine(&now, cli_config.filter_calculator.to_program());

    let mut engine: Engine = Engine::new(tasks, filter_machine, score_machine, now);

    let EngineOutput {
        mutated_tasks,
        display_tasks,
    } = engine.run(parse_cli_args(args.iter().skip(1))?)?;

    if !mutated_tasks.is_empty() {
        for task in mutated_tasks {
            put_task(&*cli_config, &task)?;
            if let Some(on_edit_cmd) = cli_config
                .hooks
                .as_ref()
                .and_then(|config| config.on_edit.as_ref())
            {
                run_string_as_shell_command(&on_edit_cmd)?;
            }
        }

        if let Some(after_cmd) = cli_config
            .hooks
            .as_ref()
            .and_then(|config| config.after.as_ref())
        {
            print!("{}", run_string_as_shell_command(&after_cmd)?);
        }
    }

    render_engine_output(display_tasks, &cli_config)?;

    Ok(())
}
