#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate clap;
extern crate hypertask_engine;

use chrono::prelude::*;
use clap::{App, Arg, SubCommand};
use hypertask_cli_context::*;
use hypertask_engine::prelude::*;
use rand::prelude::*;
use rocket::config::{Config, Environment};
use rocket::State;
use rocket_contrib::json::Json;
use std::sync::RwLock;

#[get("/")]
fn get_tasks(context: State<RwLock<CliContext>>) -> HyperTaskResult<Json<Vec<Task>>> {
    let mut task_iterator = context.read().unwrap().get_task_iterator()?;

    let task_vec = task_iterator.collect::<HyperTaskResult<Vec<Task>>>()?;

    Ok(Json(task_vec))
}

#[post("/", data = "<task_json>")]
fn post_task(
    context: State<RwLock<CliContext>>,
    task_json: Json<Task>,
) -> HyperTaskResult<Json<()>> {
    let Json(task) = task_json;

    context.write().unwrap().put_task(&task)?;

    Ok(Json(()))
}

fn run() -> HyperTaskResult<()> {
    let cli_context = CliContext::new_for_server()?;

    let port = &cli_context.get_server_port().unwrap_or(4232);

    let default_address = ("localhost".to_owned());
    let address = cli_context
        .get_server_address()
        .as_ref()
        .unwrap_or(&default_address);

    #[cfg(debug_assertions)]
    let rocket_environment = Environment::Development;

    #[cfg(not(debug_assertions))]
    let rocket_environment = Environment::Production;

    let config = Config::build(rocket_environment)
        .address(address)
        .port(*port)
        .finalize()
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Run)
                .msg("could not start rocket server, for some reason")
        })?;

    rocket::custom(config)
        .manage(RwLock::new(cli_context))
        .mount("/", routes![get_tasks, post_task])
        .launch();

    Ok(())
}

fn main() {
    let output = run();

    if let Err(e) = output {
        print_error_chain(&e)
    }
}
