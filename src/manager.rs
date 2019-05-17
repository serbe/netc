use actix_web::actix::*;

use crate::db::DBSaver;
use crate::worker::{Worker, Job};

pub struct Msg {
    pub id: usize,
    pub msg: String,
}

impl Message for Msg {
    type Result = ();
}

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
        Manager::create(move |_| {
            Manager { workers, db, ip, target }
        })
    }
}

/// Make actor from `ChatServer`
impl Actor for Manager {
    type Context = Context<Self>;
}

impl Handler<UrlList> for Manager {
    type Result = ();

    fn handle(&mut self, msg: UrlList, _: &mut Context<Self>) {
        for url in msg.list {
            self.workers.do_send(Job{ db: self.db.clone(), proxy_url: url, target_url: self.target.clone(), ip: self.ip.clone() })
        }
    }
}

/// Handler for Disconnect message.
//impl Handler<Disconnect> for Manager {
//    type Result = ();
//
//    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
//        println!("Someone disconnected");
//    }
//}

impl Handler<Msg> for Manager {
    type Result = ();

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) {
        println!("message manager id: {} msg: {}", msg.id, msg.msg);
        //        self.send_message(msg.msg.as_str(), msg.id);
    }
}
