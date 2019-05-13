//! `ClientSession` is an actor, it manages peer tcp connection and
//! proxies commands from peer to `ChatServer`.
use actix::prelude::*;
use std::io;
use std::time::{Duration, Instant};
use tokio_io::io::WriteHalf;
use tokio_tcp::TcpStream;

//use crate::codec::{ClientCodec, ClientRequest, ClientResponse};
use crate::manager::{self, Manager};

/// Chat server sends this messages to session
#[derive(Message)]
pub struct Message(pub String);

pub struct Worker {
    id: usize,
    addr: Addr<Manager>,
    hb: Instant,
}

impl Actor for Worker {
    type Context = actix::Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("worker {} started", self.id);
    }

    //    fn started(&mut self, ctx: &mut Self::Context) {
    //        // we'll start heartbeat process on session start.
    //        self.hb(ctx);
    //
    //        // register self in chat server. `AsyncContext::wait` register
    //        // future within context, but context waits until this future resolves
    //        // before processing any other events.
    //        self.addr
    //            .send(manager::Connect {
    //                addr: ctx.address(),
    //            })
    //            .into_actor(self)
    //            .then(|res, act, ctx| {
    //                match res {
    //                    Ok(res) => act.id = res,
    //                    // something is wrong with chat server
    //                    _ => ctx.stop(),
    //                }
    //                actix::fut::ok(())
    //            })
    //            .wait(ctx);
    //    }
    //
    //    fn stopping(&mut self, _: &mut Self::Context) -> Running {
    //        // notify chat server
    //        self.addr.do_send(manager::Disconnect { id: self.id });
    //        Running::Stop
    //    }
}

//impl actix::io::WriteHandler<io::Error> for Worker {}
//
///// To use `Framed` with an actor, we have to implement `StreamHandler` trait
//impl StreamHandler<ClientRequest, io::Error> for Worker {
//    /// This is main event loop for client requests
//    fn handle(&mut self, msg: ClientRequest, ctx: &mut Self::Context) {
//        //        match msg {
//        //            ChatRequest::Join(name) => {
//        //                println!("Join to room: {}", name);
//        //                self.addr.do_send(manager::Join {
//        //                    id: self.id,
//        //                    name: name.clone(),
//        //                });
//        //                //                self.framed.write(ChatResponse::Joined(name));
//        //            }
//        //            ChatRequest::Message(message) => {
//        //                // send message to chat server
//        //                println!("Peer message: {}", message);
//        //                self.addr.do_send(manager::Message {
//        //                    id: self.id,
//        //                    msg: message,
//        //                    //                    room: self.room.clone(),
//        //                })
//        //            }
//        //            // we update heartbeat time on ping from peer
//        //            ChatRequest::Ping => self.hb = Instant::now(),
//        //        }
//    }
//}

/// Handler for Message, chat server sends this message, we just send string to
/// peer
impl Handler<Message> for Worker {
    type Result = ();

    fn handle(&mut self, msg: Message, _: &mut Self::Context) {
        println!("message worker {} msg ( {} )", self.id, msg.0);
        // send message to peer
        //        self.framed.write(ChatResponse::Message(msg.0));
    }
}

/// Helper methods
impl Worker {
    pub fn new(id: usize, addr: Addr<Manager>) -> Worker {
        Worker {
            id: id,
            addr,
            hb: Instant::now(),
        }
    }

    // helper method that sends ping to client every second.
    //
    // also this method check heartbeats from client
    //    fn hb(&self, ctx: &mut actix::Context<Self>) {
    //        ctx.run_later(Duration::new(1, 0), |act, ctx| {
    //            // check client heartbeats
    //            if Instant::now().duration_since(act.hb) > Duration::new(10, 0) {
    //                // heartbeat timed out
    //                println!("Client heartbeat failed, disconnecting!");
    //
    //                // notify chat server
    //                act.addr.do_send(manager::Disconnect { id: act.id });
    //
    //                // stop actor
    //                ctx.stop();
    //            }
    //
    //            //            act.framed.write(ChatResponse::Ping);
    //            act.hb(ctx);
    //        });
    //    }
}
