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
use ansi_term::Style;
use hypertask_engine::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

pub fn run_cli(args: &[String]) -> HyperTaskResult<()> {
    let mut cli_context = CliContext::new()?;

    let tasks = cli_context
        .get_task_iterator()?
        .map(|task_result| task_result.map(|task| (task.get_id().clone(), Rc::new(task))))
        .collect::<HyperTaskResult<HashMap<Rc<Id>, Rc<Task>>>>()?;

    let mut engine: Engine = Engine::new(
        tasks,
        cli_context.get_score_machine()?,
        cli_context.get_filter_machine()?,
        cli_context.get_now(),
    );

    let EngineOutput {
        mutated_tasks,
        display_tasks,
    } = engine.run(parse_cli_args(args.iter().skip(1))?)?;

    if mutated_tasks.len() > 0 {
        for task in mutated_tasks {
            cli_context.put_task(&task)?;
        }

        cli_context.finalise_mutations()?;
    }

    let renderable_tasks = display_tasks
        .iter()
        .map(|(filtered, score, task)| {
            (Style::new(), {
                let mut map = HashMap::new();
                map.insert("id", format!("{}", task.get_id()));
                map.insert("score", format!("{0:.4}", score));
                map.insert(
                    "description",
                    format!(
                        "{}",
                        task.get_description().as_ref().unwrap_or(&"".to_string())
                    ),
                );
                map
            })
        })
        .collect();

    render_table(
        &cli_context.get_render_columns()[..],
        &Style::new().underline(),
        &renderable_tasks,
    );

    Ok(())
}
