use actix_web::actix::*;

use crate::db::DBSaver;
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
