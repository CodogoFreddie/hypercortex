#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate chrono_english;
extern crate hypertask_engine;
extern crate render_simple_cli_table;
extern crate shellexpand;

mod config;
mod parse_args;
mod render;

use crate::config::CliConfig;
use crate::parse_args::parse_cli_args;
use crate::render::render_engine_output;
use chrono::prelude::*;
use hypertask_engine::prelude::*;
use persisted_task_client::PersistedTaskClient;
use simple_persist_data::prelude::*;
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

fn get_input_tasks() -> HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>> {
    let mut hm = HashMap::new();

    for id in PersistedTaskClient::get_all_ids()? {
        let PersistedTaskClient(task) = PersistedTaskClient::load_from_storage(&id)?;
        hm.insert(Rc::new(id), Rc::new(task));
    }

    Ok(hm)
}

pub fn run_cli(args: &[String]) -> HyperTaskResult<()> {
    env_logger::init();

    let cli_config: CliConfig = CliConfig::load_from_storage()?;

    if !CliConfig::exists_in_storage()? {
        cli_config.save_to_storage()?;
    }

    let tasks: HashMap<Rc<Id>, Rc<Task>> = get_input_tasks()?;
    let now = Utc::now();
    let score_machine = create_stack_machine(&now, cli_config.score_calculator.to_program());
    let filter_machine = create_stack_machine(&now, cli_config.filter_calculator.to_program());

    let mut engine: Engine = Engine::new(tasks, filter_machine, score_machine, now);

    let EngineOutput {
        mutated_tasks,
        display_tasks,
    } = engine.run(parse_cli_args(args.iter().skip(1))?)?;

    std::mem::drop(engine);

    if !mutated_tasks.is_empty() {
        for task in mutated_tasks {
            PersistedTaskClient(task.as_ref().clone()).save_to_storage()?;
        }
    }

    render_engine_output(display_tasks, &cli_config)?;

    Ok(())
}
