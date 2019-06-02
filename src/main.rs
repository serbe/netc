use crate::db::{get_connection, DBSaver};
use crate::manager::Manager;
use crate::netutils::my_ip;
use crossbeam::channel::{select, unbounded, Receiver, Sender};
use futures::future::ok;
use futures::Stream;
use hyper::rt::Future;
use hyper::service::service_fn;
use hyper::Method;
use hyper::{Body, Request, Response, Server};
use sled::Db;
use std::thread;

mod db;
mod manager;
mod netutils;
mod utils;
mod worker;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;

fn serve(req: Request<Body>, manager: Sender<String>) -> ResponseFuture {
    // let addr = req.state().manager.clone();
    // let db = req.state().db.clone();
    // req.body()
    //     .from_err()
    //     .and_then(move |bytes: Bytes| {
    //         if let Cow::Borrowed(utf8_string) = String::from_utf8_lossy(&bytes.to_vec()) {
    //             let list: Vec<String> = utf8_string
    //                 .split('\n')
    //                 .filter_map(|s| match db.set(s, b"") {
    //                     Ok(None) => Some(s.trim().to_string()),
    //                     _ => None,
    //                 })
    //                 .collect();
    //             println!("send {} proxies", list.len());
    //             addr.do_send(manager::UrlList { list })
    //         }
    //         Ok(HttpResponse::Ok().finish())
    //     })
    //     .responder()
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/paste") => {
            println!("bingo!\n {:?}", req.headers());
            Box::new(
                req.into_body()
                    .concat2() // Concatenate all chunks in the body
                    .from_err()
                    .and_then(move |entire_body| {
                        let strs: Vec<String> = String::from_utf8(entire_body.to_vec())?
                            .split('\n')
                            .map(|s| s.trim().to_string())
                            .collect();
                        let msg = strs.len().to_string();
                        for s in strs {
                            let _ = manager.send(s);
                        }
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
    // let sys = System::new("Actix");
    // let cfg = utils::get_config();
    // let db = get_connection(&cfg.db);
    // let s_db = Db::start_default("pr_db").unwrap();
    // let ip = my_ip().unwrap();
    // let saver = DBSaver::new(db).start();
    // let bind_addr = cfg.server;
    // let num_workers = cfg.workers;
    // let target = cfg.target;
    // let manager = Manager::new(saver, ip, target, num_workers);
    // server::new(move || {
    //     App::with_state(State {
    //         db: s_db.clone(),
    //         manager: manager.clone(),
    //     })
    //     .resource("/paste", |r| r.method(Method::POST).f(paste))
    // })
    // .bind(bind_addr)
    // .unwrap()
    // .start();
    // sys.run();
    let addr = ([127, 0, 0, 1], 3000).into();
    let (manager_s, worker_r) = unbounded();
    let (worker_s, saver_r) = unbounded();
    let mut threads = Vec::new();
    threads.push(thread::spawn(move || {
        println!("saver running");
        loop {
            select! {
                recv(saver_r) -> msg => {
                    if let Ok((id, m)) = msg {
                        println!("saver got {} from {}", m, id);
                    }
                }
            }
        }
    }));
    for i in 0..10 {
        let s = worker_s.clone();
        let r = worker_r.clone();
        threads.push(thread::spawn(move || {
            let w = Worker::new(i, s, r);
            w.run();
        }));
    }
    //    for i in 0..1000 {
    //        manager_s.send(i).unwrap();
    //    }

    let service = move || {
        let sender = manager_s.clone();
        service_fn(move |req| serve(req, sender.clone()))
    };

    let server = Server::bind(&addr)
        .serve(service)
        .map_err(|e| eprintln!("server error: {}", e));

    // Run this server for... forever!
    hyper::rt::run(server);
}
