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

#[get("/")]
fn get_tasks(context: State<CliContext>) -> Json<()> {
    Json(())
}

#[post("/", data = "<task>")]
fn put_task(context: State<CliContext>, task: Json<Task>) -> Json<()> {
    //println!("{:?}", context.data_dir);

    //Json(Task::generate(&mut cli_context))
    Json(())
}

fn run() -> HyperTaskResult<()> {
    let cli_context = CliContext::new()?;

    let server_config = cli_context
        .get_config()
        .get_server_config()
        .as_ref()
        .ok_or(
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Read)
                .msg("config.toml does not contain a [server] section"),
        )?;

    let port = server_config.get_port().unwrap_or(1234);

    let config = Config::build(Environment::Staging)
        //.address("1.2.3.4")
        .port(port)
        .finalize()
        .map_err(|e| {
            HyperTaskError::new(HyperTaskErrorDomain::Context, HyperTaskErrorAction::Run)
                .msg("could not start rocket server, for some reason")
        })?;

    rocket::custom(config)
        .manage(cli_context)
        .mount("/", routes![get_tasks])
        .launch();

    Ok(())
}

fn main() {
    let output = run();

    if let Err(e) = output {
        print_error_chain(&e)
    }
}
