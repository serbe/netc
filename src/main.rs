//use actix::dev::{MessageResponse, ResponseChannel};
use actix::prelude::*;
//use core::borrow::Borrow;
//use futures::Future;

struct Msg(String);

impl Message for Msg {
    type Result = ();
}

struct Command;

impl Message for Command {
    type Result = ();
}

struct Manager {
    childs: Vec<Addr<Worker>>,
}

impl Actor for Manager {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        for i in (0..5) {
            let worker = Worker::new(i, ctx.address()).start();
            self.childs.push(worker);
        }
    }
}

impl Manager {
    fn new() -> Self {
        Manager { childs: Vec::new() }
    }
}

impl Handler<Msg> for Manager {
    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Manager>) {
        println!("msg: {}", msg.0);
    }
}

impl Handler<Command> for Manager {
    type Result = ();

    fn handle(&mut self, _msg: Command, ctx: &mut Context<Manager>) {
        for worker in self.childs.iter() {
            worker.do_send(Msg(format!("hello from manager")));
        }
    }
}

struct Worker {
    id: usize,
    manager: Addr<Manager>,
}

impl Actor for Worker {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        println!("worker {} started", self.id);
        self.manager
            .do_send(Msg(format!("hello from {} worker", self.id)));
    }
}

impl Worker {
    fn new(id: usize, manager: Addr<Manager>) -> Self {
        Worker { id, manager }
    }
}

impl Handler<Msg> for Worker {
    type Result = ();

    fn handle(&mut self, msg: Msg, _ctx: &mut Context<Worker>) {
        println!("msg: {}", msg.0);
    }
}

fn main() {
    System::run(|| {
        let manager = Manager::new();
        let addr = manager.start();
        addr.do_send(Command);
    });
}
