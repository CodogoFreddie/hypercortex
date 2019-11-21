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

use crate::context::{CliContext, RenderColumns};
use crate::parse_args::parse_cli_args;
use crate::render::render_table;
use ansi_term::Colour::{Black, Cyan, Green, Red, Yellow};
use ansi_term::Style;
use hypertask_engine::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;

fn style_task(filtered: &bool, score: &f64) -> Style {
    let mut style = Style::new();

    if score > &10.0 {
        style = style.fg(Green);
    };

    if score > &20.0 {
        style = style.fg(Cyan);
    };

    if score > &30.0 {
        style = style.fg(Red);
    };

    if *filtered {
        style = style.dimmed();
    };

    style
}

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
            (style_task(filtered, score), {
                let mut map = HashMap::new();
                map.insert(RenderColumns::Id, format!("{}", task.get_id()));
                map.insert(RenderColumns::Score, format!("{0:.4}", score));
                map.insert(
                    RenderColumns::Description,
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
    )?;

    Ok(())
}
