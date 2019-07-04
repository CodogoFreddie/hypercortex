use crate::tag::{Sign, Tag};
use crate::task::Task;
use std::collections::{HashMap, HashSet};

const GUTTER_WIDTH: usize = 2;

const HEADER_ORDER: &[&str] = &["id", "description", "tags"];

pub fn render_table(tasks: &Vec<Task>) -> () {
    let mut widths = HashMap::<&str, usize>::new();
    let mut hash_mapped_tasks: Vec<HashMap<&str, String>> = vec![];

    for task in tasks {
        let hash_map = task.to_renderable_hash_map();;
        for (key, value) in &hash_map {
            let length = value.len();

            widths.entry(key).or_insert(length);

            let current_length = widths.get(key).unwrap();

            if (current_length < &length) {
                widths.insert(key, length);
            }
        }
        hash_mapped_tasks.push(hash_map)
    }

    for task in hash_mapped_tasks {
        for header in HEADER_ORDER {
            print!(
                "{:1$}",
                task.get(header).unwrap(),
                widths.get(header).unwrap() + GUTTER_WIDTH
            );
        }
        println!("");
    }

}
