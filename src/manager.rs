use crate::db::DBSaver;
use crate::worker::{Job, Worker};
use actix_web::actix::{Actor, Addr, Context, Handler, Message, SyncArbiter};

pub struct UrlList {
    pub list: Vec<String>,
}

impl Message for UrlList {
    type Result = ();
}

pub struct Manager {
    workers: Addr<Worker>,
    db: Addr<DBSaver>,
    ip: String,
    target: String,
}

impl Manager {
    pub fn new(db: Addr<DBSaver>, ip: String, target: String, num_workers: usize) -> Addr<Manager> {
        let workers: Addr<Worker> = SyncArbiter::start(num_workers, || Worker);
        Manager::create(move |_| Manager {
            workers,
            db,
            ip,
            target,
        })
    }
}

impl Actor for Manager {
    type Context = Context<Self>;
}

impl Handler<UrlList> for Manager {
    type Result = ();

    fn handle(&mut self, msg: UrlList, _: &mut Context<Self>) {
        for url in msg.list {
            if url.contains("://") {
                self.workers.do_send(Job {
                    db: self.db.clone(),
                    proxy_url: url,
                    target_url: self.target.clone(),
                    ip: self.ip.clone(),
                });
            } else {
                self.workers.do_send(Job {
                    db: self.db.clone(),
                    proxy_url: format!("http://{}", url),
                    target_url: self.target.clone(),
                    ip: self.ip.clone(),
                });
                self.workers.do_send(Job {
                    db: self.db.clone(),
                    proxy_url: format!("socks5://{}", url),
                    target_url: self.target.clone(),
                    ip: self.ip.clone(),
                });
            }
        }
    }
}
