// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]

use futures::future::Future;
use crossbeam_channel::{unbounded, Receiver, Sender, TryRecvError};
use futures::future::lazy;
use std::time::Duration;
use std::thread;

mod db;
mod netutils;

enum Msg {
    Data(String),
    Error(String),
    // Terminate,
}

struct Config {
    db: String,
    target: String,
    workers: usize,
}

fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    let workers = dotenv::var("workers")
        .expect("No found variable workers like 4 in environment").parse::<usize>().expect("wrong variable workers in environment");
    Config { db, target, workers }
}

fn worker(id: usize, sr: Sender<Msg>, rw: Receiver<Msg>, target: &str, my_ip: &str) {
    loop {
        match rw.try_recv() {
            Ok(Msg::Data(s)) => match netutils::get_checked_proxy(&s, target, my_ip) {
                Ok(p) => sr.send(Msg::Data(p.hostname)).unwrap(),
                Err(e) => sr.send(Msg::Error(e.to_string())).unwrap(),
            },
            Ok(Msg::Error(e)) => println!("{} rw recv {}", id, e),
            // Ok(Msg::Terminate) => break,
            Err(TryRecvError::Disconnected) => break,
            Err(TryRecvError::Empty) => (),
        }
        thread::sleep(Duration::new(0, 50000));
    }
}

fn getter(rr: Receiver<Msg>) {
    loop {
        match rr.try_recv() {
            Ok(Msg::Data(s)) => {
                println!("received a message: {:?}", s);
            },
            Ok(Msg::Error(s)) => {
                println!("received error: {:?}", s);
            },
            _ => (),
        }
        thread::sleep(Duration::new(0, 50000));
    }
}

fn main() {
    let config = get_config();
    let my_ip = netutils::my_ip().unwrap();
    println!("my ip is {}", &my_ip);
    let conn = db::get_connection(&config.db);

    let thread_pool = tokio_threadpool::Builder::new()
        // .pool_size(config.workers)
        .before_stop(|| {
            println!("thread stopping");
        })
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
            println!("worker {} started", i);
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

    thread_pool.shutdown().wait().unwrap();
}
