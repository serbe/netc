// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel::{bounded, Receiver, Sender, TryRecvError};
use futures::future::lazy;
// use futures::Future;
use std::time::Duration;

use std::thread;
// use tokio_threadpool::Builder;
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
}

fn get_config() -> Config {
    let db = dotenv::var("db")
        .expect("No found variable db like postgres://postgres@localhost:5433 in environment");
    let target = dotenv::var("target")
        .expect("No found variable target like http://targethost:433/path in environment");
    Config { db, target }
}

//fn bg_task() -> impl Future<Item = (), Error = ()> {
//    let (tx, rx) = mpsc::channel(1_024);
//
//    tokio::spawn({
//        stream::iter_ok(0..10)
//            .fold(tx, |tx, i| {
//                tx.send(format!("Message {} from spawned task", i))
//                    .map_err(|e| println!("error = {:?}", e))
//            })
//            .map(|_| ()) // Drop tx handle
//    });
//
//    rx.for_each(|msg| {
//        println!("Got `{}`", msg);
//        Ok(())
//    })
//}

fn start_worker(id: usize, sx: Sender<Msg>, rx: Receiver<Msg>, target: &str, my_ip: &str) {
    loop {
        match rx.try_recv() {
            Ok(Msg::Data(s)) => {
                // println!("worker {} get {}", id, value);
                match netutils::get_checked_proxy(&s, target, my_ip) {
                    Ok(p) => {
                        // println!("receive {:?}", p);
                        sx.send(Msg::Data(p.hostname)).unwrap();
                    }
                    Err(e) => {
                        println!("worker {} in get_checked_proxy {}", id, e.to_string());
                        sx.send(Msg::Error(e.to_string())).unwrap();
                    }
                }
            }
            Err(TryRecvError::Disconnected) => break,
            _ => (),
        }
    }
}

fn main() {
    let config = get_config();
    let my_ip = netutils::my_ip().unwrap();
    println!("my ip is {}", &my_ip);
    let conn = db::get_connection(&config.db);

    let n_workers = 4;
    // // let n_jobs = 40;

    let thread_pool = tokio_threadpool::Builder::new()
        .pool_size(n_workers)
        .before_stop(|| {
            println!("thread stopping");
        })
        //     // .keep_alive(Some(time::Duration::from_secs(30)))
        .build();

    let (sw, rw) = bounded(20);
    let (sr, rr) = bounded(20);

    for i in 0..n_workers {
        let sr = sr.clone();
        let rw = rw.clone();
        let target = config.target.clone();
        let my_ip = my_ip.clone();
        thread_pool.spawn(lazy(move || {
            println!("worker {} started", i);
            start_worker(i, sr, rw, &target, &my_ip);
            Ok(())
        }));
    }

    let proxies = db::get_all_n_proxy(conn, 20).unwrap();
    // println!("get proxy {:?}", proxies);
    for proxy in proxies {
        // println!("send {}", &proxy.hostname);
        sw.send(Msg::Data(proxy.hostname)).unwrap();
    }
    // sw.send(Msg::Terminate).unwrap();
    // sw.send(Msg::Terminate).unwrap();
    // sw.send(Msg::Terminate).unwrap();
    // sw.send(Msg::Terminate).unwrap();
    drop(sw);
    // for i in 0..n_jobs {
    //     tx.send(i).unwrap();
    // }

    // let t_pool = tokio_threadpool::Builder::new()
    // .pool_size(n_workers)
    // .keep_alive(Some(time::Duration::from_secs(30)))
    // .build();

    // t_pool.spawn(lazy(|| {
    //     println!("start sleep");
    //     thread::sleep(time::Duration::from_secs(11));
    //     println!("end sleep");
    //     // thread_pool.shutdown_now();
    //     drop(tx);
    //     Ok(())
    // }));

    // let mut sel = Select::new();
    // let roper = sel.recv(&rr);

    let mut i = 1;
    let c: Vec<_> = rr
        .iter()
        .filter_map(|msg| match msg {
            Msg::Data(s) => {
                i += 1;
                println!("{} received a message: {:?}", i, s);
                Some(s)
            }
            Msg::Error(s) => {
                i += 1;
                println!("{} received error: {:?}", i, s);
                None
            }
        })
        .collect();
    // match err {
    // TryRecvError::Disconnected => {
    // println!("disconnected");
    // break;
    // }
    // thread::sleep(Duration::from_millis(500));
    // },
    // }
    // thread_pool.shutdown().wait().unwrap();
}
