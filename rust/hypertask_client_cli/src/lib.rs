#[macro_use]
extern crate lazy_static;
extern crate ansi_term;
extern crate chrono_english;
extern crate hypertask_config_file_opener;
extern crate hypertask_engine;
extern crate shellexpand;

mod context;
mod parse_args;
mod render;

use crate::context::CliContext;
use crate::parse_args::parse_cli_args;
use crate::render::render_table;
use hypertask_engine::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_cli(args: &[String]) -> HyperTaskResult<()> {
    let cli_context = CliContext::new()?;

    let tasks = cli_context
        .get_task_iterator()?
        .map(|task_result| task_result.map(|task| (task.get_id().clone(), Rc::new(task))))
        .collect::<HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>>>()?;

    let mut engine: Engine = Engine::new(
        parse_cli_args(args.iter().skip(1))?,
        tasks,
        cli_context.get_score_machine()?,
        cli_context.get_filter_machine()?,
        cli_context.get_now(),
    );

    engine.run()?;

    //render_table(&tasks_to_display);

    Ok(())
}
