use crate::proxy::{check_proxy, Proxy};
use crossbeam::channel::{select, Receiver, Sender};
use std::thread;

pub struct Worker {
    pub id: usize,
    pub ip: String,
    pub target: String,
    pub server: Receiver<String>,
    pub db_saver: Sender<Proxy>,
}

impl Worker {
    fn new(
        id: usize,
        ip: String,
        target: String,
        server: Receiver<String>,
        db_saver: Sender<Proxy>,
    ) -> Self {
        Worker {
            id,
            ip,
            target,
            server,
            db_saver,
        }
    }

    pub fn start(
        num_workers: usize,
        ip: String,
        target: String,
        worker_r: Receiver<String>,
        worker_s: Sender<Proxy>,
    ) {
        for i in 0..num_workers {
            let r = worker_r.clone();
            let s = worker_s.clone();
            let ip = ip.clone();
            let target = target.clone();
            thread::spawn(move || {
                let worker = Worker::new(i, ip, target, r, s);
                worker.run();
            });
        }
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.server) -> msg => {
                    if let Ok(proxy_url) = msg {
                        if let Ok(proxy) = check_proxy(&proxy_url, &self.target, &self.ip) {
                            let _ = self.db_saver.send(proxy);
                        }
                    }
                }
            }
        }
    }
}
