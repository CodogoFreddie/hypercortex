use ansi_term::Colour::{Cyan, Red};
use ansi_term::Style;
use chrono::prelude::*;
use hypertask_engine::prelude::*;
use std::collections::HashMap;

const GUTTER_WIDTH: usize = 2;

const HEADER_ORDER: &[&str] = &["id", "score", "description", "tags", "due", "recur"];

fn render_score_to_significant_figures(score: f64, figures: i32) -> String {
    let exponent = score.log10().floor() as i32;
    let x = 10.0_f64.powi(exponent - figures - 1);
    let precision = i32::max(figures - 1 - exponent, 0);
    let output = (score / x).floor() * x;

    format!("{:.precision$}", output, precision = precision as usize)
}

fn task_to_renderable_hash_map(finalised_task: &FinalisedTask) -> HashMap<&str, String> {
    let mut hm = HashMap::<&str, String>::new();
    let task = finalised_task.get_task();

    let Id(id) = task.get_id();
    hm.insert("id", id.to_string());

    hm.insert(
        "score",
        render_score_to_significant_figures(*finalised_task.get_score(), 3),
    );

    if let Some(description) = task.get_description() {
        hm.insert("description", description.to_string());
    }

    hm.insert("tags", {
        let mut tags_vec = task
            .get_tags()
            .iter()
            .map(|t| format!("+{}", t))
            .collect::<Vec<String>>();

        tags_vec.sort();

        tags_vec.join(" ")
    });

    if let Some(due) = task.get_due() {
        hm.insert("due", due.format("%Y-%m-%d %H:%M").to_string());
    }

    if let Some(recur) = task.get_recur() {
        hm.insert("recur", format!("{}", recur));
    }

    hm
}

pub fn render_table(finalised_tasks: &[FinalisedTask]) {
    let now = Utc::now();
    let mut widths = HashMap::<&str, usize>::new();
    let mut hash_mapped_tasks: Vec<(HashMap<&str, String>, &Task)> = vec![];

    //let lines = 20;
    let lines = if let Some((_, height)) = term_size::dimensions() {
        height - 5
    } else {
        40
    };

    //calculate column widths
    for header in HEADER_ORDER {
        widths.insert(header, header.len());
    }
    for finalised_task in finalised_tasks.iter().take(lines) {
        let hash_map = task_to_renderable_hash_map(&finalised_task);
        for (key, value) in &hash_map {
            let length = value.len();
            let current_length = widths[key];

            if current_length < length {
                widths.insert(key, length);
            }
        }
        hash_mapped_tasks.push((hash_map, finalised_task.get_task()))
    }

    //print the header
    let header_string = HEADER_ORDER
        .iter()
        .map(|header| {
            widths.entry(header).or_insert(1);
            format!("{:1$}", header, widths[header] + GUTTER_WIDTH)
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", Style::new().underline().paint(header_string));

    //print the tasks
    for (task_hash, task) in hash_mapped_tasks {
        let task_string = HEADER_ORDER
            .iter()
            .map(|header| {
                format!(
                    "{:1$}",
                    if let Some(val) = task_hash.get(header) {
                        val
                    } else {
                        ""
                    },
                    widths[header] + GUTTER_WIDTH
                )
            })
            .collect::<Vec<String>>()
            .join("");
        println!(
            "{}",
            if task.is_overdue(&now) {
                Red.paint(task_string).to_string()
            } else if task.is_soon_due(&now) {
                Cyan.paint(task_string).to_string()
            } else {
                task_string
            }
        );
    }
}
