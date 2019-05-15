use crate::manager::Manager;
use crate::netutils::my_ip;
use crate::saver::DBSaver;
use actix_web::{
    actix::*, http::Method, server, App, AsyncResponder, Error, HttpMessage, HttpRequest,
    HttpResponse,
};
use bytes::Bytes;
use futures::future::Future;
use rpdb::get_connection;
use std::borrow::Cow;

mod manager;
mod netutils;
mod saver;
mod utils;
mod worker;

struct State {
    manager: Addr<Manager>,
}

fn paste(req: &HttpRequest<State>) -> Box<Future<Item = HttpResponse, Error = Error>> {
    let addr = req.state().manager.clone();
    req.body()
        .from_err()
        .and_then(move |bytes: Bytes| {
            if let Cow::Borrowed(utf8_string) = String::from_utf8_lossy(&bytes.to_vec()) {
                let list = utf8_string
                    .split('\n')
                    .map(std::string::ToString::to_string)
                    .collect();
                addr.do_send(manager::UrlList { list })
            }
            Ok(HttpResponse::Ok().finish())
        })
        .responder()
}

fn main() -> std::io::Result<()> {
    let cfg = utils::get_config();
    let db = get_connection(&cfg.db);
    let saver = DBSaver::new(db).start();
    let bind_addr = cfg.server;
    let num_workers = cfg.workers;
    let ip = my_ip()?;
    let manager = Manager::new(saver, ip, num_workers);
    let server = server::new(move || {
        App::with_state(State {
            manager: manager.clone(),
        })
        .resource("/paste", |r| r.method(Method::POST).f(paste))
    })
    .bind(bind_addr)?;
    server.run();
    Ok(())
}
