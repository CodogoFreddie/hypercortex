#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate hypertask_engine;
extern crate clap;

use chrono::prelude::*;
use clap::{Arg, App, SubCommand};
use hypertask_engine::prelude::*;
use rand::prelude::*;
use rocket::State;
use rocket_contrib::json::Json;
use serde_json;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::path::Path;
use std::process::Command;
use std::sync::atomic::AtomicUsize;
use std::fs;

pub struct CliContext {}

impl GetNow for CliContext {
    fn get_now(&self) -> DateTime<Utc> {
        Utc::now()
    }
}

impl GenerateId for CliContext {
    fn generate_id(&mut self) -> String {
        let mut result = String::new();

        for _ in 0..NUMBER_OF_CHARS_IN_FULL_ID {
            let random = VALID_ID_CHARS
                .chars()
                .choose(&mut thread_rng())
                .expect("Couldn't get random char");

            result.push(random);
        }

        result
    }
}

#[derive(Debug)]
struct Context {
    data_dir: String,
    pre_hook: Option<String>,
    post_hook: Option<String>,
}

#[get("/")]
fn get_tasks(context: State<Context>) -> Result<Json<Vec<Task>>, String> {
    println!("{:?}", context.data_dir);

    let mut cli_context = CliContext {};

    let paths = fs::read_dir(&context.data_dir)
        .map_err(|_| format!("folder {} could not be found", &context.data_dir.to_string()))?;

    let tasks: Vec<Task> = paths.map(|path| {
        path.map_err(|_| "Failed to open task".to_string())
            .and_then(|file_path| {
                File::open(file_path.path())
                    .map_err(|_| format!("failed to open task `{:?}`", file_path))
                    .and_then(|file| {
                        serde_json::from_reader::<std::io::BufReader<std::fs::File>, Task>(
                            BufReader::new(file),
                        )
                        .map_err(|_| format!("failed to parse task @ `{:?}`", file_path))
                    })
            })
    }).collect::<Result<Vec<Task>, String>>()?;

    Ok(Json(tasks))
}

#[post("/", data = "<task>")]
fn put_task(context: State<Context>, task: Json<Task>) -> Json<Task> {
    println!("{:?}", context.data_dir);

    let mut cli_context = CliContext {};

    Json(Task::generate(&mut cli_context))
}

fn main() {
    let matches = App::new("hypertask REST server")
        .version("0.1")
        .arg(Arg::with_name("DATA_DIR")
             .short("d")
             .long("data-dir")
             .value_name("DATA_DIR")
             .help("The directory where tasks are stored")
             .takes_value(true)
             .required(true)
         )
        .arg(Arg::with_name("PRE_HOOK")
             .long("pre-hook")
             .value_name("PRE_HOOK")
             .help("Shell command to run before making a modification to the stored task")
             .takes_value(true)
         )
        .arg(Arg::with_name("POST_HOOK")
             .long("post-hook")
             .value_name("POST_HOOK")
             .help("Shell command to run after making a modification to the stored task")
             .takes_value(true)
         )
        .get_matches();

    rocket::ignite()
    .manage(Context {
        data_dir: matches.value_of("DATA_DIR").map(|s| s.to_owned()).unwrap(),
        pre_hook: matches.value_of("PRE_HOOK").map( |s| s.to_owned()),
        post_hook: matches.value_of("POST_HOOK").map( |s| s.to_owned()),
    })
    
    .mount("/", routes![get_tasks]).launch();
}
