use crate::db::DBSaver;
use crate::netutils::check_proxy;
use actix_web::actix::{Actor, Addr, Handler, Message, SyncContext};

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

impl Actor for Worker {
    type Context = SyncContext<Self>;
}

impl Handler<Job> for Worker {
    type Result = ();

    fn handle(&mut self, job: Job, _: &mut Self::Context) {
        match check_proxy(&job.proxy_url, &job.target_url, &job.ip) {
            Ok(proxy) => {
                // println!("ok: {}", proxy.hostname);
                job.db.do_send(proxy);
            }
            Err(_) => {
                // println!("err: {}", job.proxy_url);
            }
        }
    }
}
