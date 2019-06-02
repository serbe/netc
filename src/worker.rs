use crate::db::Proxy;
use crate::netutils::check_proxy;
use crossbeam::channel::{select, Receiver, Sender};

pub struct Worker {
    pub id: usize,
    pub ip: String,
    pub target: String,
    pub manager: Receiver<String>,
    pub db_saver: Sender<Proxy>,
}

impl Worker {
    pub fn new(
        id: usize,
        ip: String,
        target: String,
        manager: Receiver<String>,
        db_saver: Sender<Proxy>,
    ) -> Self {
        Worker {
            id,
            ip,
            target,
            manager,
            db_saver,
        }
    }

    pub fn run(&self) {
        loop {
            select! {
                recv(self.manager) -> msg => {
                    if let Ok(proxy_url) = msg {
                        if let Ok(proxy) = check_proxy(&proxy_url, &self.target, &self.ip) {
                            self.db_saver.send(proxy);
                        }
                    }
                }
            }
        }
    }
}
