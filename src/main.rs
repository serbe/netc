use futures::future::lazy;
use futures::sync::{mpsc, oneshot};
use futures::{future, stream, Future, Sink, Stream};
use std::time::Duration;
use tokio_threadpool::{Builder, ThreadPool};

fn bg_task() -> impl Future<Item = (), Error = ()> {
    let (tx, rx) = mpsc::channel(1_024);

    tokio::spawn({
        stream::iter_ok(0..10)
            .fold(tx, |tx, i| {
                tx.send(format!("Message {} from spawned task", i))
                    .map_err(|e| println!("error = {:?}", e))
            })
            .map(|_| ()) // Drop tx handle
    });

    rx.for_each(|msg| {
        println!("Got `{}`", msg);
        Ok(())
    })
}

fn main() {
    let pool = ThreadPool::new();
    let (tx, rx) = oneshot::channel();

    pool.spawn(lazy(|| {
        println!("Running on the pool");
        tx.send("complete")
            .map_err(|e| println!("send error, {}", e))
    }));

    println!("Result: {:?}", rx.wait());
    pool.shutdown().wait().unwrap();

    tokio::run(lazy(|| {
        tokio::spawn(lazy(move || bg_task()));
        for i in 0..40 {
            tokio::spawn(lazy(move || {
                println!("Hello from task {}", i);
                Ok(())
            }));
        }

        Ok(())
    }));

    let pool = ThreadPool::new();
    let tx = pool.sender().clone();

    let res = oneshot::spawn(
        future::lazy(|| {
            println!("Running on the pool");
            Ok::<_, ()>("complete")
        }),
        &tx,
    );

    println!("Result: {:?}", res.wait());

    let thread_pool = Builder::new()
        .pool_size(4)
        .keep_alive(Some(Duration::from_secs(30)))
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

    thread_pool.spawn(lazy(|| {
        println!("called from a worker thread");
        Ok(())
    }));

    // Gracefully shutdown the threadpool
    thread_pool.shutdown().wait().unwrap();
}
