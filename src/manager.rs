use actix_web::actix::*;
use std::collections::HashMap;

//use crate::netutils::Url;
use crate::saver::DBSaver;
use crate::worker;
use crate::worker::Worker;

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
    workers: HashMap<usize, Addr<worker::Worker>>,
}

impl Default for Manager {
    fn default() -> Manager {
        Manager {
            workers: HashMap::new(),
        }
    }
}

impl Manager {
    pub fn new(saver: Addr<DBSaver>, ip: String, num_workers: usize) -> Addr<Manager> {
        Manager::create(move |ctx| {
            let mut workers: HashMap<usize, Addr<Worker>> = HashMap::new();
            for i in 0..num_workers {
                let worker = Worker::new(i, ctx.address(), saver, ip).start();
                //                worker.do_send(worker::ManagerMsg("heLLo".to_string()));
                workers.insert(i, worker);
            }
            Manager { workers }
        })
    }

    // Send message to all users in the room
    //    fn send_message(message: &str, skip_id: usize) {
    //        if let Some(sessions) = self.rooms.get(room) {
    //            for id in sessions {
    //                if *id != skip_id {
    //                    if let Some(addr) = self.sessions.get(id) {
    //                        addr.do_send(worker::Message(message.to_owned()))
    //                    }
    //                }
    //            }
    //        }
    //    }
}

/// Make actor from `ChatServer`
impl Actor for Manager {
    type Context = Context<Self>;
}

impl Handler<UrlList> for Manager {
    type Result = ();

    fn handle(&mut self, msg: UrlList, _: &mut Context<Self>) {
        println!("get {} urls", msg.list.len());
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

/// Handler for Message message.
impl Handler<Msg> for Manager {
    type Result = ();

    fn handle(&mut self, msg: Msg, _: &mut Context<Self>) {
        println!("message manager id: {} msg: {}", msg.id, msg.msg);
        //        self.send_message(msg.msg.as_str(), msg.id);
    }
}
