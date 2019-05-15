use actix_web::actix::*;
use std::time::Instant;

use crate::db::DBSaver;
use crate::manager::Manager;

pub struct ManagerMsg(pub String);

impl Message for ManagerMsg {
    type Result = ();
}

pub struct Worker {
    id: usize,
    addr: Addr<Manager>,
    saver: Addr<DBSaver>,
    ip: String,
    hb: Instant,
}

impl Actor for Worker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("worker {} started", self.id);
    }
}

impl Handler<ManagerMsg> for Worker {
    type Result = ();

    fn handle(&mut self, msg: ManagerMsg, _: &mut Self::Context) {
        println!("message worker {} msg ( {} )", self.id, msg.0);
        // send message to peer
        //        self.framed.write(ChatResponse::Message(msg.0));
    }
}

/// Helper methods
impl Worker {
    pub fn new(id: usize, addr: Addr<Manager>, saver: Addr<DBSaver>, ip: String) -> Worker {
        Worker {
            id,
            addr,
            saver,
            ip,
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
