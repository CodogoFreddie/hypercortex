use crate::task::Task;
use ansi_term::Style;
use std::collections::HashMap;

const GUTTER_WIDTH: usize = 2;

const HEADER_ORDER: &[&str] = &["id", "description", "tags", "due", "recur"];

pub fn render_table(tasks: &Vec<Task>) -> () {
    let mut widths = HashMap::<&str, usize>::new();
    let mut hash_mapped_tasks: Vec<HashMap<&str, String>> = vec![];

    //calculate column widths
    for task in tasks {
        let hash_map = task.to_renderable_hash_map();;
        for (key, value) in &hash_map {
            let length = value.len();

            widths.entry(key).or_insert(length);

            let current_length = widths.get(key).unwrap();

            if current_length < &length {
                widths.insert(key, length);
            }
        }
        hash_mapped_tasks.push(hash_map)
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
    for task in hash_mapped_tasks {
        let task_string = HEADER_ORDER
            .iter()
            .map(|header| {
                format!(
                    "{:1$}",
                    if let Some(val) = task.get(header) {
                        val
                    } else {
                        ""
                    },
                    widths.get(header).unwrap() + GUTTER_WIDTH
                )
            })
            .collect::<Vec<String>>()
            .join("");
        println!("{}", task_string);
    }
}
