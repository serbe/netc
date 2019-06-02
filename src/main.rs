use crate::db::{get_connection, DBSaver};
use crate::manager::Manager;
use crate::netutils::my_ip;
use crate::worker::Worker;
use crossbeam::channel::{unbounded, Sender};
use futures::future::ok;
use futures::Stream;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::{Body, Method, Request, Response, Server};

mod db;
mod manager;
mod netutils;
mod utils;
mod worker;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<dyn Future<Item = Response<Body>, Error = GenericError> + Send>;

fn serve(req: Request<Body>, manager: Sender<Vec<String>>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/paste") => {
            println!("bingo!\n {:?}", req.headers());
            Box::new(
                req.into_body()
                    .concat2()
                    .from_err()
                    .and_then(move |entire_body| {
                        let url_list: Vec<String> = String::from_utf8(entire_body.to_vec())?
                            .split('\n')
                            .map(|s| s.trim().to_string())
                            .collect();
                        let msg = url_list.len().to_string();
                        let _ = manager.send(url_list);
                        let response = Response::builder().status(200).body(Body::from(msg))?;
                        Ok(response)
                    }),
            )
        }
        _ => {
            let response = Response::builder().status(405).body(Body::empty()).unwrap();
            Box::new(ok(response))
        }
    }
}

fn main() {
    let cfg = utils::get_config();
    let db = get_connection(&cfg.db);
    let sled = cfg.sled;
    let ip = my_ip().unwrap();
    let bind_addr = cfg.server.parse().unwrap();
    let num_workers = cfg.workers;
    let target = cfg.target;
    let (server_s, manager_r) = unbounded();
    let (manager_s, worker_r) = unbounded();
    let (worker_s, saver_r) = unbounded();

    Manager::start(manager_r, manager_s, sled);

    Worker::start(num_workers, ip, target, worker_r, worker_s);

    DBSaver::start(db, saver_r);

    let service = move || {
        let server_s = server_s.clone();
        service_fn(move |req| serve(req, server_s.clone()))
    };

    let server = Server::bind(&bind_addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);
}
