use actix_web::actix::*;
use rpdb::{Connection, Proxy};

pub struct DBSaver {
    pub db: Connection,
}

impl Actor for DBSaver {
    type Context = Context<Self>;
}

impl DBSaver {
    pub fn new(db: Connection) -> Self {
        DBSaver { db }
    }
}

impl Message for Proxy {
    type Result = ();
}

impl Handler<Proxy> for DBSaver {
    type Result = ();

    fn handle(&mut self, proxy: Proxy, _: &mut Self::Context) {
        //    let mut proxy = Proxy::from(proxy_url)?;
        //    proxy.work = true;
        //    if !body.contains(my_ip) && body.matches("<p>").count() == 1 {
        //        proxy.anon = true;
        //    }
        //    Ok(proxy)
        //        println!("message worker {} msg ( {} )", self.id, msg.0);
        // send message to peer
        //        self.framed.write(ChatResponse::Message(msg.0));
    }
}
