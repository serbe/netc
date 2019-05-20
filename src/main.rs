use crate::db::{get_connection, DBSaver};
use crate::manager::Manager;
use crate::netutils::my_ip;
use actix_web::{
    actix::{Actor, Addr, System},
    http::Method,
    server, App, AsyncResponder, Error, HttpMessage, HttpRequest, HttpResponse,
};
use bytes::Bytes;
use futures::future::Future;
use sled::Db;
use std::borrow::Cow;

mod db;
mod manager;
mod netutils;
mod utils;
mod worker;

struct State {
    db: Db,
    manager: Addr<Manager>,
}

fn paste(req: &HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let addr = req.state().manager.clone();
    let db = req.state().db.clone();
    req.body()
        .from_err()
        .and_then(move |bytes: Bytes| {
            if let Cow::Borrowed(utf8_string) = String::from_utf8_lossy(&bytes.to_vec()) {
                let list = utf8_string
                    .split('\n')
                    .filter_map(|s| match db.set(s, b"") {
                        Ok(None) => Some(s.trim().to_string()),
                        _ => None,
                    })
                    .collect();
                addr.do_send(manager::UrlList { list })
            }
            Ok(HttpResponse::Ok().finish())
        })
        .responder()
}

fn main() {
    let sys = System::new("Actix");
    let cfg = utils::get_config();
    let db = get_connection(&cfg.db);
    let s_db = Db::start_default("pr_db").unwrap();
    let ip = my_ip().unwrap();
    let saver = DBSaver::new(db).start();
    let bind_addr = cfg.server;
    let num_workers = cfg.workers;
    let target = cfg.target;
    let manager = Manager::new(saver, ip, target, num_workers);
    server::new(move || {
        App::with_state(State {
            db: s_db.clone(),
            manager: manager.clone(),
        })
        .resource("/paste", |r| r.method(Method::POST).f(paste))
    })
    .bind(bind_addr)
    .unwrap()
    .start();
    sys.run();
}
