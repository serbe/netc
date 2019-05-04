// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use crossbeam_channel::{select, unbounded, Receiver, Sender};
use rpdb::Proxy;
use futures::stream::Stream;
use futures::{
    future,
    future::{lazy, Future},
};
// use hyper::rt::Future;
use hyper::client::HttpConnector;
use hyper::service::service_fn;
use hyper::{header, Body, Chunk, Client, Method, Request, Response, Server, StatusCode};
// use std::thread;
// use std::time::Duration;

mod netutils;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type ResponseFuture = Box<Future<Item = Response<Body>, Error = GenericError> + Send>;

enum Msg {
    Data(String),
    Error(String),
    Terminate,
}

struct Config {
    db: String,
    port: u16,
    target: String,
    workers: usize,
}

fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let port = dotenv::var("port")
        .expect("No found variable port like 8080 in environment")
        .parse::<u16>()
        .expect("wrong variable port in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    let workers = dotenv::var("workers")
        .expect("No found variable workers like 4 in environment")
        .parse::<usize>()
        .expect("wrong variable workers in environment");
    Config {
        db,
        port,
        target,
        workers,
    }
}

fn worker(
    id: usize,
    sr: Sender<Result<Proxy, String>>,
    rw: Receiver<Msg>,
    target: &str,
    my_ip: &str,
) {
    loop {
        select! {
            recv(rw) -> data => match data {
                Ok(Msg::Data(s)) => sr.send(netutils::check_proxy(&s, target, my_ip)).unwrap(),
                Ok(Msg::Error(e)) => println!("{} rw recv {}", id, e),
                Ok(Msg::Terminate) => break,
                // Err(TryRecvError::Disconnected) => break,
                // Err(TryRecvError::Empty) => (),
                _ => (),
            }
            // thread::sleep(Duration::new(0, 50000));
        }
    }
}

fn getter(rr: Receiver<Result<Proxy, String>>) {
    loop {
        select! {
            recv(rr) -> data => match data {
                Ok(proxy) => println!("received proxy: {:?}", proxy),
                Err(s) => println!("received error: {:?}", s),
            }
        }
    }
}

fn post_response(req: Request<Body>) -> ResponseFuture {
    // A web api to run against
    Box::new(
        req.into_body()
            .concat2() // Concatenate all chunks in the body
            .from_err()
            .and_then(|entire_body| {
                // TODO: Replace all unwraps with proper error handling
                let body = String::from_utf8(entire_body.to_vec())?;
                println!("{}", body);
                let response = Response::builder()
                    .status(StatusCode::OK)
                    // .body(Body::from("моя работает"))?;
                .body(Body::empty())?;
                Ok(response)
            }),
    )
}

fn response(req: Request<Body>, client: &Client<HttpConnector>) -> ResponseFuture {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/paste") => post_response(req),
        _ => {
            // let body = Body::from("Not Found");
            Box::new(future::ok(
                Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .unwrap(),
            ))
        }
    }
}

fn main() {
    let config = get_config();

    let my_ip = netutils::my_ip().unwrap();
    println!("my ip is {}", &my_ip);
    let conn = rpdb::get_connection(&config.db);

    let thread_pool = tokio_threadpool::Builder::new()
        .pool_size(config.workers + 1)
        // .before_stop(|| {
        //     println!("thread stopping");
        // })
        //     // .keep_alive(Some(time::Duration::from_secs(30)))
        .build();

    let (sw, rw) = unbounded();
    let (sr, rr) = unbounded();

    for i in 0..config.workers {
        let sr = sr.clone();
        let rw = rw.clone();
        let target = config.target.clone();
        let my_ip = my_ip.clone();
        thread_pool.spawn(lazy(move || {
            worker(i, sr, rw, &target, &my_ip);
            Ok(())
        }));
    }

    // let proxies = db::get_n_work_proxy(conn, 20).unwrap();
    // for proxy in proxies {
    //     sw.send(Msg::Data(proxy.hostname)).unwrap();
    // }

    // let t_pool = tokio_threadpool::Builder::new()
    // .pool_size(n_workers)
    // .keep_alive(Some(time::Duration::from_secs(30)))
    // .build();

    thread_pool.spawn(lazy(move || {
        // let mut i = 0;
        getter(rr);
        Ok(())
    }));

    let addr = ([127, 0, 0, 1], config.port).into();

    hyper::rt::run(future::lazy(move || {
        // Share a `Client` with all `Service`s
        let client = Client::new();

        let service = move || {
            // Move a clone of `client` into the `service_fn`.
            let client = client.clone();
            service_fn(move |req| response(req, &client))
        };

        let server = Server::bind(&addr)
            .serve(service)
            .map_err(|e| eprintln!("server error: {}", e));

        server
    }));

    thread_pool.shutdown().wait().unwrap();
}
