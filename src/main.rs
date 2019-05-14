//use actix::dev::{MessageResponse, ResponseChannel};
use actix_web::{actix::*, http::Method, server, App, HttpRequest, HttpResponse, Responder};
//use futures::Stream;
use manager::Manager;
//use std::io::Read;
//use std::net;
//use std::str::FromStr;
//use tokio_codec::FramedRead;
//use tokio_io::AsyncRead;
//use tokio_tcp::{TcpListener, TcpStream};

mod manager;
mod utils;
mod worker;

struct State {
    manager: Addr<Manager>,
}
//
//impl Actor for CheckServer {
//    type Context = Context<Self>;
//}

fn paste(req: &HttpRequest<State>) -> impl Responder {
    println!("{:?}", req.headers());
    req.state().manager.do_send(manager::Text {
        msg: "1".to_string(),
    });
    //    println!("body:\n {}", req);
    //    format!("body:\n {}", req);
    HttpResponse::NotFound()
}

fn main() -> std::io::Result<()> {
    let cfg = utils::get_config();

    let bind_addr = cfg.server.clone();
    let num_workers = cfg.workers.clone();

    let manager = Manager::new(num_workers);

    let server = server::new(move || {
        App::with_state(State {
            manager: manager.clone(),
        })
        .resource("/paste", |r| r.method(Method::POST).f(paste))
    })
    .bind(bind_addr)?;
    Ok(server.run())

    //    System::run(move || {
    //        let addr = net::SocketAddr::from_str(&bind_addr).unwrap();
    //        let listener = TcpListener::bind(&addr).unwrap();
    //
    //        CheckServer::create(|ctx| {
    //            ctx.add_message_stream(listener.incoming().map_err(|_| ()).map(|mut st| {
    //                let mut body = String::new();
    //                st.read_to_string(&mut body);
    //                println!("{}", body);
    //                Ok(())
    //            }));
    //            Server { manager }
    //        });
    //    })

    // System::run(|| {
    //     let manager = Manager::new();
    //     let addr = manager.start();
    //     addr.do_send(Command);
    // })
}
