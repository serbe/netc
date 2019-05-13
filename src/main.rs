//use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
use actix::prelude::*;
use futures::Stream;
use manager::Manager;
use std::net;
use std::str::FromStr;
use tokio_codec::FramedRead;
use tokio_io::AsyncRead;
use tokio_tcp::{TcpListener, TcpStream};

mod manager;
mod utils;
mod worker;

struct Server {
    manager: Addr<Manager>,
}

impl Actor for Server {
    type Context = Context<Self>;
}

fn main() -> std::io::Result<()> {
    let cfg = utils::get_config();

    let bind_addr = cfg.server.clone();
    let num_workers = cfg.workers.clone();

    System::run(move || {
        let manager = Manager::new(num_workers);
        let addr = net::SocketAddr::from_str(&bind_addr).unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        // Server::create(|ctx| {
        //     ctx.add_message_stream(listener.incoming().map_err(|_| ()).map(|st| {
        //         let addr = st.peer_addr().unwrap();
        //         TcpConnect(st, addr)
        //     }));
        //     Server { manager }
        // });
    })

    // System::run(|| {
    //     let manager = Manager::new();
    //     let addr = manager.start();
    //     addr.do_send(Command);
    // })
}
