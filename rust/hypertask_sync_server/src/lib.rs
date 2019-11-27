extern crate hypertask_engine;

mod config;
mod io;

use crate::config::SyncServerConfig;
use crate::io::get_input_tasks;
use actix_web::get;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

type TaskHashes = HashMap<Rc<Id>, u64>;

#[get("/hashes")]
fn greet(data: web::Data<SyncServerConfig>, _req: HttpRequest) -> impl Responder {
    let mut task_hashes = TaskHashes::new();

    let input_tasks: HashMap<Rc<Id>, Rc<Task>> =
        get_input_tasks(&data).expect("could not get tasks");

    for (id, task) in input_tasks.iter() {
        task_hashes.insert(id.clone(), task.calculate_hash());
    }

    web::Json(task_hashes)
}

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-server.toml")?;
    let config_file_getter: Arc<ConfigFileGetter<SyncServerConfig>> =
        Arc::new(config_file_opener.parse()?);
    let config_file_getter_instance = config_file_getter.clone();
    let sync_server_config: &SyncServerConfig = config_file_getter.get_config();

    dbg!(&sync_server_config);

    HttpServer::new(move || {
        let sync_server_config: &SyncServerConfig = config_file_getter_instance.get_config();

        App::new().data(sync_server_config.clone()).service(greet)
    })
    .bind((
        sync_server_config.hostname.as_str(),
        sync_server_config.port,
    ))
    .expect("could not start server")
    .run()
    .unwrap();

    format!(
        "started server @ {}:{}",
        sync_server_config.hostname, sync_server_config.port,
    );

    Ok(())
}
