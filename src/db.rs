use chrono::{DateTime, Local};
use crossbeam::channel::{select, Receiver};
use postgres::{Connection, TlsMode};
use std::thread;

#[derive(Debug)]
pub struct Proxy {
    pub insert: bool,
    pub update: bool,
    pub work: bool,
    pub anon: bool,
    pub checks: i32,
    pub hostname: String,
    pub host: String,
    pub port: String,
    pub scheme: String,
    pub create_at: DateTime<Local>,
    pub update_at: DateTime<Local>,
    pub response: i64,
}

impl Proxy {
    pub fn from(s: &str) -> Result<Self, String> {
        let raw = s;

        if raw.contains('#') {
            Err(format!("hostname contain fragment {}", raw))?
        }

        if raw.contains('?') {
            Err(format!("hostname contain query {}", raw))?
        }

        let (raw, scheme) = if let Some(pos) = raw.find("://") {
            (
                raw.get(pos + 3..)
                    .ok_or_else(|| format!("not parse scheme {}", raw))?,
                raw.get(..pos)
                    .ok_or_else(|| format!("not parse scheme {}", raw))?
                    .to_string(),
            )
        } else {
            Err(format!("hostname not contain scheme {}", raw))?
        };

        if raw.contains('@') {
            Err(format!("user info in hostname not supported {}", raw))?
        };

        if raw.contains('/') {
            Err(format!("hostname contain path {}", raw))?
        };

        let (host, port) = if let Some(pos) = raw.rfind(':') {
            if let Some(start) = raw.find('[') {
                if let Some(end) = raw.find(']') {
                    if start == 0 && pos == end + 1 {
                        (
                            raw.get(..pos)
                                .ok_or_else(|| format!("not parse host {}", raw))?
                                .to_string(),
                            raw.get(pos + 1..)
                                .ok_or_else(|| format!("not parse port {}", raw))?
                                .to_string(),
                        )
                    } else {
                        Err(format!("not parse ipv6 {}", raw))?
                    }
                } else {
                    Err(format!("not parse ipv6 {}", raw))?
                }
            } else {
                (
                    raw.get(..pos)
                        .ok_or_else(|| format!("not parse host {}", raw))?
                        .to_string(),
                    raw.get(pos + 1..)
                        .ok_or_else(|| format!("not parse port {}", raw))?
                        .to_string(),
                )
            }
        } else {
            Err(format!("not parse port {}", raw))?
        };

        let _ = port
            .parse::<u32>()
            .map_err(|_| format!("not parse port {}", port))?;

        Ok(Proxy {
            insert: false,
            update: false,
            work: false,
            anon: false,
            checks: 0,
            hostname: format!("{}:{}", host, port),
            host,
            port,
            scheme,
            create_at: chrono::Local::now(),
            update_at: chrono::Local::now(),
            response: 0,
        })
    }
}

pub struct DBSaver {
    pub db: Connection,
    pub workers: Receiver<Proxy>,
}

impl DBSaver {
    fn new(db: Connection, workers: Receiver<Proxy>) -> Self {
        DBSaver { db, workers }
    }

    pub fn start(db: Connection, workers: Receiver<Proxy>) {
        let db_saver = DBSaver::new(db, workers);
        thread::spawn(move || db_saver.run());
    }

    fn run(&self) {
        loop {
            select! {
                recv(self.workers) -> msg => {
                    if let Ok(proxy) = msg {
                        let _ = insert_or_update(&self.db, proxy);
                    }
                }
            }
        }
    }
}

pub fn get_connection(params: &str) -> Connection {
    Connection::connect(params, TlsMode::None).unwrap()
}

pub fn insert_or_update(conn: &Connection, proxy: Proxy) -> Result<u64, String> {
    conn.execute(
        "INSERT INTO
            proxies (work, anon, checks, hostname, host, port, scheme, create_at, update_at, response)
        VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT
            (hostname)
        DO UPDATE SET
            (work, anon, checks, update_at, response) =
            ($1, $2, $3 + 1, $9, $10)
        ",
        &[&proxy.work, &proxy.anon, &proxy.checks, &proxy.hostname, &proxy.host, &proxy.port, &proxy.scheme, &proxy.create_at, &proxy.update_at, &proxy.response]).map_err(|e| format!("error insert {}", e.to_string()))
}
