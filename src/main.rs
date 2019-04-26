use futures::future::lazy;
use futures::Future;
//use std::time::Duration;
use tokio_threadpool::Builder;

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

fn main() {
    //    let pool = ThreadPool::new();
    //    let (tx, rx) = oneshot::channel();
    //
    //    pool.spawn(lazy(|| {
    //        println!("Running on the pool");
    //        tx.send("complete")
    //            .map_err(|e| println!("send error, {}", e))
    //    }));

    //    println!("Result: {:?}", rx.wait());
    //    pool.shutdown().wait().unwrap();
    //
    //    tokio::run(lazy(|| {
    //        tokio::spawn(lazy(move || bg_task()));
    //        for i in 0..40 {
    //            tokio::spawn(lazy(move || {
    //                println!("Hello from task {}", i);
    //                Ok(())
    //            }));
    //        }
    //
    //        Ok(())
    //    }));

    //    let pool = ThreadPool::new();
    //    let tx = pool.sender().clone();
    //
    //    let res = oneshot::spawn(
    //        future::lazy(|| {
    //            println!("Running on the pool");
    //            Ok::<_, ()>("complete")
    //        }),
    //        &tx,
    //    );
    //
    //    println!("Result: {:?}", res.wait());

    let thread_pool = Builder::new()
        .pool_size(4)
        //        .keep_alive(Some(Duration::from_secs(30)))
        .around_worker(|worker, _| {
            println!("worker {:?} is starting up", worker.id());
            worker.run();
            println!("worker {:?} is shutting down", worker.id());
        })
        .after_start(|| {
            println!("thread started");
        })
        .before_stop(|| {
            println!("thread stopping");
        })
        .build();

    for i in 0..40 {
        thread_pool.spawn(lazy(move || {
            println!("Hello from task {}", i);
            Ok(())
        }));
    }

    // Gracefully shutdown the threadpool
    thread_pool.shutdown().wait().unwrap();
}
