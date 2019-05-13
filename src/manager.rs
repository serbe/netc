use actix::prelude::*;
use rand::{self, Rng};
use std::collections::{HashMap, HashSet};

use crate::worker;
use crate::worker::Worker;

pub struct Connect {
    pub addr: HashMap<usize, Addr<worker::Worker>>,
}

impl actix::Message for Connect {
    type Result = usize;
}

#[derive(Message)]
pub struct Disconnect {
    pub id: usize,
}

#[derive(Message)]
pub struct Msg {
    pub id: usize,
    pub msg: String,
}

/// `ChatServer` manages chat rooms and responsible for coordinating chat
/// session. implementation is super primitive
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
    pub fn new(num_workers: usize) -> Addr<Manager> {
        Manager::create(move |ctx| {
            let mut workers: HashMap<usize, Addr<Worker>> = HashMap::new();
            for i in 0..num_workers {
                let worker = Worker::new(i, ctx.address()).start();
                worker.send(worker::Message("heLLo".to_string()));
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

/// Handler for Connect message.
///
/// Register new session and assign unique id to this session
//impl Handler<Connect> for Manager {
//    type Result = usize;
//
//    fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
//        println!("Someone joined");
//
//        // notify all users in same room
//        //        self.send_message(&"Main".to_owned(), "Someone joined", 0);
//
//        // register session with random id
//        let id = rand::thread_rng().gen::<usize>();
//        self.workers.insert(id, msg.addr);
//
//        //        // auto join session to Main room
//        //        self.rooms.get_mut(&"Main".to_owned()).unwrap().insert(id);
//
//        // send id back
//        id
//    }
//}

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
