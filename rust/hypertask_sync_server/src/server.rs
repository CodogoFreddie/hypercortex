use crate::cli_args::CliArgs;
use crate::sync_secret;
use futures::future::BoxFuture;
use hypertask_engine::prelude::*;
use hypertask_task_io_operations::{delete_task, get_input_tasks, get_task, put_task};
use std::collections::HashMap;
use std::error::Error;
use std::rc::Rc;
use time::Duration;

type TaskHashes = HashMap<Rc<Id>, u64>;
type ServerState = (CliArgs, sync_secret::SyncSecret);
type ServerWithState = tide::Server<ServerState>;

fn get_available_port() -> Option<u16> {
    (10000..20000).find(|port| port_is_available(*port))
}

fn port_is_available(port: u16) -> bool {
    match std::net::TcpListener::bind(("127.0.0.1", port)) {
        Ok(_) => true,
        Err(_) => false,
    }
}

struct AuthMiddleware();

impl tide::Middleware<ServerState> for AuthMiddleware {
    fn handle<'a>(
        &'a self,
        req: tide::Request<ServerState>,
        next: tide::Next<'a, ServerState>,
    ) -> BoxFuture<'a, tide::Response> {
        Box::pin(async move {
            let (_, secret) = req.state();

            let auth_header_option: Option<String> =
                req.header("Authorization").map(|s| s.to_owned());

            if let Some(auth_header) = auth_header_option {
                if format!("hypertask {}", secret) != auth_header {
                    error!("invalid Authorization header provided: `{}`", auth_header);

                    tide::Response::new(401)
                        .body_string("incorect Authorization header provided".to_owned())
                } else {
                    next.run(req).await
                }
            } else {
                error!("no Authorization header provided");
                tide::Response::new(400).body_string("no Authorization header provided".to_owned())
            }
        })
    }
}

fn attach_get_hashes(app: &mut ServerWithState) {
    info!("attached GET /hashes");

    app.at("/hashes").get(
        |mut req: tide::Request<(CliArgs, sync_secret::SyncSecret)>| async move {
            info!("GET /hashes");

            let (config, _) = req.state();

            let mut task_hashes = TaskHashes::new();
            let input_tasks: HashMap<Rc<Id>, Rc<Task>> =
                get_input_tasks(config).expect("could not get tasks");

            for (id, task) in input_tasks.iter() {
                task_hashes.insert(id.clone(), task.calculate_hash());
            }

            tide::Response::new(200).body_json(&task_hashes).unwrap()
        },
    );
}

fn attach_post_task(app: &mut ServerWithState) {
    info!("attached POST /task/:id");

    app.at("/task/:id").post(
        |mut req: tide::Request<(CliArgs, sync_secret::SyncSecret)>| async move {
            info!(
                "POST /task/{}",
                req.param::<String>("id")
                    .unwrap_or_else(|_| "__NULL__".to_string())
            );

            let config = req.state().0.clone();

            let task_id = match req.param::<String>("id") {
                Ok(task_id) => {
                    info!("task_id: {:?}", task_id);
                    task_id
                }
                Err(e) => {
                    error!("task_id error: {}", e);
                    return tide::Response::new(400).body_string("no id provided".to_owned());
                }
            };

            let client_task: Option<Task> = match req.body_json().await {
                Ok(client_task) => {
                    info!("client_task: {:?}", &client_task);
                    client_task
                }
                Err(e) => {
                    error!("client_task error: {}", e);
                    return tide::Response::new(400)
                        .body_string("invalid task recieved".to_owned());
                }
            };

            let server_task: Option<Task> = match get_task(&config, &Id(task_id)) {
                Ok(server_task) => {
                    info!("server_task: {:?}", &server_task);
                    server_task
                }
                Err(e) => {
                    error!("server_task error: {}", e);
                    return tide::Response::new(500)
                        .body_string("could not read local task".to_owned());
                }
            };

            let resolved_task: Option<Task> =
                match Task::resolve_task_conflict(client_task, server_task) {
                    Ok(resolved_task) => {
                        info!("resolved_task: {:?}", &resolved_task);
                        resolved_task
                    }
                    Err(e) => {
                        error!("resolved_task error: {}", e);
                        return tide::Response::new(400)
                            .body_string("tasks did not match".to_owned());
                    }
                };

            if let Some(reified_resolved_task) = &resolved_task {
                info!("updating local task");

                if let Err(e) = put_task(&config, reified_resolved_task) {
                    error!("updating local task {:?}", e);

                    return tide::Response::new(500)
                        .body_string("could not write local task".to_owned());
                }
            }

            tide::Response::new(200).body_json(&resolved_task).unwrap()
        },
    );
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

    app.middleware(AuthMiddleware());

    attach_get_hashes(&mut app);
    attach_post_task(&mut app);

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
