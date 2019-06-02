use crossbeam::channel::{select, Receiver, Sender};

pub struct Manager {
    server: Receiver<Vec<String>>,
    workers: Sender<String>,
}

impl Manager {
    pub fn new(server: Receiver<Vec<String>>, workers: Sender<String>) -> Self {
        Manager { server, workers }
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.server) -> msg => {
                    if let Ok(url_list) = msg {
                        for url in url_list {
                            if url.contains("://") {
                                self.workers.send(url);
                            } else {
                                self.workers.send(format!("http://{}", url));
                                self.workers.send(format!("socks5://{}", url));
                            }
                        }

                    }
                }
            }
        }
    }
}
