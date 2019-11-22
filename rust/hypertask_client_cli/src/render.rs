use crate::context::CliContext;
use ansi_term::Colour::{Cyan, Green, Red};
use ansi_term::Style;
use chrono::prelude::*;
use hypertask_engine::prelude::*;
use render_simple_cli_table::render_table;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum RenderColumns {
    Id,
    Score,
    Description,
    Blocked,
    Tags,
    Due,
    Recur,
}

impl fmt::Display for RenderColumns {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(match self {
            RenderColumns::Id => "Id",
            RenderColumns::Score => "Score",
            RenderColumns::Description => "Description",
            RenderColumns::Blocked => "Blocked By",
            RenderColumns::Tags => "Tags",
            RenderColumns::Due => "Due",
            RenderColumns::Recur => "Recur",
        })
    }
}

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

fn format_date_time(dt: DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M").to_string()
}

fn renderify_task(
    input: &(bool, f64, Rc<Task>),
) -> (ansi_term::Style, HashMap<RenderColumns, String>) {
    let (filtered, score, task) = input;

    let mut map = HashMap::new();
    map.insert(RenderColumns::Id, format!("{}", task.get_id()));
    map.insert(RenderColumns::Score, format!("{0:.4}", score));
    map.insert(
        RenderColumns::Blocked,
        task.get_blocked_by()
            .map(|d| format!("{}", d))
            .unwrap_or_else(String::default),
    );

    map.insert(
        RenderColumns::Recur,
        task.get_recur()
            .as_ref()
            .map(|x| format!("{}", x))
            .unwrap_or_else(String::default),
    );

    map.insert(
        RenderColumns::Due,
        task.get_due()
            .map(format_date_time)
            .unwrap_or_else(String::default),
    );

    map.insert(RenderColumns::Tags, {
        let mut vec = task
            .get_tags()
            .iter()
            .map(|tag| format!("+{}", tag))
            .collect::<Vec<String>>();
        vec.sort();
        vec.join(" ")
    });

    map.insert(
        RenderColumns::Description,
        format!(
            "{}",
            task.get_description().as_ref().unwrap_or(&"".to_string())
        ),
    );

    (style_task(filtered, score), map)
}

pub fn render_engine_output(
    display_tasks: Vec<(bool, Score, Rc<Task>)>,
    cli_context: &CliContext,
) -> HyperTaskResult<()> {
    let renderable_tasks = display_tasks.iter().map(renderify_task).collect();

    render_table(
        &cli_context.get_render_columns()[..],
        &Style::new().underline(),
        &renderable_tasks,
    )
    .map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Render, HyperTaskErrorAction::Write).from(e)
    })
}
