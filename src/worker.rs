use actix_web::actix::*;
// use std::time::Instant;

use crate::db::DBSaver;
// use crate::manager::Manager;
use crate::netutils::check_proxy;

pub struct Job {
    pub db: Addr<DBSaver>,
    pub proxy_url: String,
    pub target_url: String,
    pub ip: String,
}

impl Message for Job {
    type Result = ();
}

pub struct Worker;
//  {
//     id: usize,
//     addr: Addr<Manager>,
//     saver: Addr<DBSaver>,
//     ip: String,
//     hb: Instant,
// }

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<Job> for Worker {
    type Result = ();

    fn handle(&mut self, job: Job, _: &mut Self::Context) {
        if let Ok(proxy) = check_proxy(&job.proxy_url, &job.target_url, &job.ip) {
            job.db.do_send(proxy);
        }
    }
}

// impl Worker {
    // pub fn new(id: usize, addr: Addr<Manager>, saver: Addr<DBSaver>, ip: String) -> Worker {
    //     Worker {
    //         id,
    //         addr,
    //         saver,
    //         ip,
    //         hb: Instant::now(),
    //     }
    // }

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
// }
