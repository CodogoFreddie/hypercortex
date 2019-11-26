extern crate hypertask_engine;

mod config;

use crate::config::SyncServerConfig;
use actix_web::get;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use hypertask_config_file_opener::{ConfigFileGetter, ConfigFileOpener};
use hypertask_engine::prelude::*;
use std::collections::HashMap;

type TaskHashes = HashMap<Id, u64>;

#[get("/hashes")]
fn greet(req: HttpRequest) -> impl Responder {
    let task_hashes = TaskHashes::new();

    web::Json(task_hashes)
}

pub fn start() -> HyperTaskResult<()> {
    let mut config_file_opener = ConfigFileOpener::new("sync-server.toml")?;
    let config_file_getter: ConfigFileGetter<SyncServerConfig> = config_file_opener.parse()?;
    let sync_server_config: &SyncServerConfig = config_file_getter.get_config();

    dbg!(sync_server_config);

    HttpServer::new(|| App::new().service(greet))
        .bind((
            sync_server_config.hostname.as_str(),
            sync_server_config.port,
        ))
        .expect("Can not bind to port 8000")
        .run()
        .unwrap();

    format!(
        "started server @ {}:{}",
        sync_server_config.hostname, sync_server_config.port,
    );

    Ok(())
}
