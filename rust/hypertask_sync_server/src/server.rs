use crate::cli_args::CliArgs;
use crate::sync_secret;
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use time::Duration;

type TaskHashes = HashMap<Rc<Id>, u64>;

fn get_available_port() -> Option<u16> {
    (10000..20000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub async fn start(config: CliArgs) -> HyperTaskResult<()> {
    info!("starting http server");

    let hostname = config
        .hostname
        .as_ref()
        .map(|x| x.clone())
        .unwrap_or_else(|| {
            info!("no host name provided, falling back to `localhost`");
            "localhost".to_string()
        });

    let port = config.port.as_ref().map(|x| x.clone()).unwrap_or_else(|| {
        info!("no port provided, finding an open port");

        get_available_port().expect("could not find a port to bind to")
    });

    let secret = config
        .sync_secret
        .as_ref()
        .map(|x| x.clone())
        .unwrap_or_else(|| {
            info!("no secret provided, generating a new random secret");
            sync_secret::generate()
        });

    let mut app = tide::with_state((config, secret.clone()));

    app.at("/hashes").get(
        |mut req: tide::Request<(CliArgs, sync_secret::SyncSecret)>| async move {
            let (config, secret) = req.state();

            info!("request for current hashes");

            if let Some(auth_header) = req.header("Authorization") {
                if format!("hypertask {}", secret) != auth_header {
                    error!("invalid Authorization header provided: `{}`", auth_header);

                    tide::Response::new(401).body_string("incorect Authorization header provided".to_owned())
                } else {
                    let mut task_hashes = TaskHashes::new();
                    let input_tasks: HashMap<Rc<Id>, Rc<Task>> =
                        get_input_tasks(config).expect("could not get tasks");

                    for (id, task) in input_tasks.iter() {
                        task_hashes.insert(id.clone(), task.calculate_hash());
                    }

                    tide::Response::new(200).body_json(&task_hashes).unwrap()
                }
            } else {
                error!("no Authorization header provided");
                tide::Response::new(400)
                    .body_string("no Authorization header provided".to_owned())
            }
        },
    );

    info!(
        "listening @ http://{}:{} with secret `{}`",
        hostname, port, secret
    );

    app.listen((hostname.as_str(), port)).await.map_err(|e| {
        HyperTaskError::new(HyperTaskErrorDomain::Syncing, HyperTaskErrorAction::Run)
            .msg("could not start sync server")
            .from(e)
    })
}

//#[post("/task/{id}")]
//fn compare_tasks(
//config_data: web::Data<SyncServerConfig>,
//path: web::Path<String>,
//client_task_input: web::Json<Option<Task>>,
//req: HttpRequest,
//) -> actix_web::Result<web::Json<Option<Task>>> {
//if let Some(Ok(auth_header)) = req.headers().get("Authorization").map(|x| x.to_str()) {
//if format!("hypertask {}", &config_data.sync_secret) != auth_header {
//return Err(actix_web::error::ErrorUnauthorized(
//"invalid sync_secret provided",
//));
//}
//}

//let id = Id(path.to_string());
//let config: &SyncServerConfig = config_data.get_ref();

//let server_task: Option<Task> = get_task(config, &id).expect("could not open task");

//let client_task: Option<Task> = client_task_input.into_inner();

//let resolved_task: Option<Task> =
//Task::resolve_task_conflict(&(Utc::now() - Duration::days(30)), server_task, client_task)
//.expect("tasks did not have the same id");

//match &resolved_task {
//Some(task) => put_task(config, &task).expect("could not save task"),
//None => delete_task(config, &id).expect("could not delete task"),
//};

//Ok(web::Json(resolved_task))
//}

//#[get("/hashes")]
//fn get_hashes(
//config_data: web::Data<SyncServerConfig>,
//req: HttpRequest,
//) -> actix_web::Result<web::Json<TaskHashes>> {
//if let Some(Ok(auth_header)) = req.headers().get("Authorization").map(|x| x.to_str()) {
//if format!("hypertask {}", &config_data.sync_secret) != auth_header {
//return Err(actix_web::error::ErrorUnauthorized(
//"invalid sync_secret provided",
//));
//}
//}

//let mut task_hashes = TaskHashes::new();
//let config: &SyncServerConfig = config_data.get_ref();

//let input_tasks: HashMap<Rc<Id>, Rc<Task>> =
//get_input_tasks(config).expect("could not get tasks");

//for (id, task) in input_tasks.iter() {
//task_hashes.insert(id.clone(), task.calculate_hash());
//}

//Ok(web::Json(task_hashes))
//}

//fn get_config_object() -> HyperTaskResult<SyncServerConfig> {
//let mut config_file_opener = ConfigFileOpener::new("sync-server.toml")?;
//let config_file_getter: ConfigFileGetter<SyncServerConfig> = config_file_opener.parse()?;
//Ok(config_file_getter.get_config().clone())
//}

//pub fn start() -> HyperTaskResult<()> {
//let sync_server_config = get_config_object()?;

//println!(
//"started syncing server for dir `{}` @ http://{}:{}",
//sync_server_config
//.task_state_dir
//.to_str()
//.expect("could not read task_state_dir"),
//sync_server_config.hostname,
//sync_server_config.port,
//);

//HttpServer::new(|| {
//let config = get_config_object().expect("could not load config");

//App::new()
//.data(config)
//.service(get_hashes)
//.service(compare_tasks)
//})
//.bind((
//sync_server_config.hostname.as_str(),
//sync_server_config.port,
//))
//.expect("could not create server")
//.run()
//.expect("could not run server");

//Ok(())
//}
