// #![allow(unused_imports)]
// #![allow(dead_code)]
// #![allow(unused_variables)]
use futures::future::lazy;
use futures::Future;
//use std::time::Duration;
use crossbeam::channel::unbounded;
use crossbeam::channel::TryRecvError;

// use std::{thread, time};
// use tokio_threadpool::Builder;
mod db;
mod netutils;

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

fn start_worker(
    id: usize,
    tx: crossbeam::channel::Sender<String>,
    rx: crossbeam::channel::Receiver<String>,
    target: &str,
    my_ip: &str,
) {
    loop {
        match rx.try_recv() {
            Ok(value) => {
                println!("worker {} get {}", id, value);
                match netutils::get_checked_proxy(&value, target, my_ip) {
                    Ok(p) => {
                        println!("receive {:?}", p);
                        tx.send(p.hostname).unwrap();
                    }
                    Err(e) => println!("error in get_checked_proxy {}", e.to_string()),
                }

            }
            Err(err) => match err {
                TryRecvError::Disconnected => {
                    println!("disconnected");
                    break;
                }
                TryRecvError::Empty => (),
            },
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
        //     // .keep_alive(Some(time::Duration::from_secs(30)))
        .build();

    let (tx, rx) = unbounded();

    for i in 0..n_workers {
        let rx = rx.clone();
        let tx = tx.clone();
        let target = config.target.clone();
        let my_ip = my_ip.clone();
        thread_pool.spawn(lazy(move || {
            println!("worker {} started", i);
            start_worker(i, tx, rx, &target, &my_ip);
            Ok(())
        }));
    }

    let proxies = db::get_all_n_proxy(conn, 20).unwrap();
    println!("get proxy {:?}", proxies);
    for proxy in proxies {
        println!("send {}", &proxy.hostname);
        tx.send(proxy.hostname).unwrap();
    }
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

    // loop {
    //     match rx.try_recv() {
    //         Ok(value) => println!("received a message: {}", value),
    //         Err(err) => match err {
    //             TryRecvError::Disconnected => {
    //                 println!("disconnected");
    //                 break;
    //             }
    //             _ => (),
    //         },
    //     }
    // }
    thread_pool.shutdown().wait().unwrap();
}
