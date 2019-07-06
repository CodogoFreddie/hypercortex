use crate::task::Task;
use ansi_term::Colour::{Cyan, Red};
use ansi_term::Style;
use std::collections::HashMap;

const GUTTER_WIDTH: usize = 2;

const HEADER_ORDER: &[&str] = &["id", "description", "tags", "due", "recur"];

pub fn render_table(tasks: &Vec<Task>) -> () {
    let mut widths = HashMap::<&str, usize>::new();
    let mut hash_mapped_tasks: Vec<(HashMap<&str, String>, &Task)> = vec![];

    //calculate column widths
    for header in HEADER_ORDER {
        widths.insert(header, header.len());
    }
    for task in tasks {
        let hash_map = task.to_renderable_hash_map();;
        for (key, value) in &hash_map {
            let length = value.len();

            let current_length = widths.get(key).unwrap();

            if current_length < &length {
                widths.insert(key, length);
            }
        }
        hash_mapped_tasks.push((hash_map, task))
    }

    //print the header
    let header_string = HEADER_ORDER
        .iter()
        .map(|header| {
            widths.entry(header).or_insert(1);
            format!("{:1$}", header, widths.get(header).unwrap() + GUTTER_WIDTH)
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", Style::new().underline().paint(header_string));

    //print the tasks
    for (task_hash, task) in hash_mapped_tasks.iter().take(40) {
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
                    widths.get(header).unwrap() + GUTTER_WIDTH
                )
            })
            .collect::<Vec<String>>()
            .join("");
        println!(
            "{}",
            if task.is_overdue() {
                Red.paint(task_string).to_string()
            } else if task.is_soon_due() {
                Cyan.paint(task_string).to_string()
            } else {
                task_string
            }
        );
    }
}
