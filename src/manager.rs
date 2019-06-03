use crossbeam::channel::{select, Receiver, Sender};
use sled::Db;
use std::thread;

pub struct Manager {
    server: Receiver<Vec<String>>,
    workers: Sender<String>,
    db: Db,
}

impl Manager {
    fn new(
        server: Receiver<Vec<String>>,
        workers: Sender<String>,
        db_name: String,
    ) -> Result<Manager, String> {
        let db = Db::start_default(db_name).map_err(|e| e.to_string())?;
        Ok(Manager {
            server,
            workers,
            db,
        })
    }

    pub fn start(
        server: Receiver<Vec<String>>,
        workers: Sender<String>,
        db_name: String,
    ) -> Result<(), String> {
        let manager = Manager::new(server, workers, db_name)?;
        thread::spawn(move || manager.run());
        Ok(())
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.server) -> msg => {
                    if let Ok(url_list) = msg {
                        for url in url_list {
                            if self.db.set(url.clone(), b"") == Ok(None) {
                                if url.contains("://") {
                                    let _ = self.workers.send(url);
                                } else {
                                    let _ = self.workers.send(format!("http://{}", url));
                                    let _ = self.workers.send(format!("socks5://{}", url));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
