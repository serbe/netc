// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use crossbeam_channel::{select, unbounded, Receiver, Sender};
use db::Proxy;
use futures::future::lazy;
use futures::future::Future;
// use hyper::rt::Future;
use hyper::service::service_fn_ok;
use hyper::{Body, Response, Server};
// use std::thread;
// use std::time::Duration;

mod db;
mod netutils;

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

fn main() {
    let config = get_config();

    let my_ip = netutils::my_ip().unwrap();
    println!("my ip is {}", &my_ip);
    let conn = db::get_connection(&config.db);

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

    let proxies = db::get_n_work_proxy(conn, 20).unwrap();
    for proxy in proxies {
        sw.send(Msg::Data(proxy.hostname)).unwrap();
    }

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
    let new_svc = || service_fn_ok(|_req| Response::new(Body::from("Hello, World!")));

    let server = Server::bind(&addr)
        .serve(new_svc)
        .map_err(|e| eprintln!("server error: {}", e));

    hyper::rt::run(server);

    thread_pool.shutdown().wait().unwrap();
}
