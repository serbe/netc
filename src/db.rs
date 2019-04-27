use chrono::NaiveDateTime;
use postgres::{rows::Row, Connection, TlsMode};

#[derive(Debug)]
pub struct Proxy {
    pub insert: bool,
    pub update: bool,
    pub work: bool,
    pub anon: bool,
    pub checks: i64,
    pub hostname: String,
    pub host: String,
    pub port: String,
    pub scheme: String,
    pub create_at: NaiveDateTime,
    pub update_at: NaiveDateTime,
    pub response: i64,
}

fn full_from_row(row: Row) -> Result<Proxy, &str> {
    Ok(Proxy {
        insert: false,
        update: false,
        work: row
            .get_opt(0)
            .ok_or_else(|| "error get work")?
            .map_err(|_| "error unwrap work")?,
        anon: row
            .get_opt(1)
            .ok_or_else(|| "error get anon")?
            .map_err(|_| "error unwrap anon")?,
        checks: row
            .get_opt(2)
            .ok_or_else(|| "error get checks")?
            .map_err(|_| "error unwrap checks")?,
        hostname: row
            .get_opt(3)
            .ok_or_else(|| "error get hostname")?
            .map_err(|_| "error unwrap hostname")?,
        host: row
            .get_opt(4)
            .ok_or_else(|| "error get host")?
            .map_err(|_| "error unwrap host")?,
        port: row
            .get_opt(5)
            .ok_or_else(|| "error get port")?
            .map_err(|_| "error unwrap port")?,
        scheme: row
            .get_opt(6)
            .ok_or_else(|| "error get scheme")?
            .map_err(|_| "error unwrap scheme")?,
        create_at: row
            .get_opt(7)
            .ok_or_else(|| "error get create_at")?
            .map_err(|_| "error unwrap create_at")?,
        update_at: row
            .get_opt(8)
            .ok_or_else(|| "error get update_at")?
            .map_err(|_| "error unwrap update_at")?,
        response: row
            .get_opt(9)
            .ok_or_else(|| "error get response")?
            .map_err(|_| "error unwrap response")?,
    })
}

pub fn get_connection(params: &str) -> Connection {
    Connection::connect(params, TlsMode::None).unwrap()
}

pub fn get_all_proxy(conn: Connection) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    if let Ok(rows) = &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies",
        &[],
    ) {
        for row in rows {
            if let Ok(proxy) = full_from_row(row) {
                proxies.push(proxy);
            }
        }
    };
    proxies
}

pub fn get_all_n_proxy(conn: Connection, n: i64) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    if let Ok(rows) = &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
        LIMIT
            $1",
        &[&n],
    ) {
        for row in rows {
            if let Ok(proxy) = full_from_row(row) {
                proxies.push(proxy);
            }
        }
    };
    proxies
}

pub fn get_all_work_proxy(conn: Connection) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    if let Ok(rows) = &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true",
        &[],
    ) {
        for row in rows {
            if let Ok(proxy) = full_from_row(row) {
                proxies.push(proxy);
            }
        }
    };
    proxies
}

pub fn get_n_work_proxy(conn: Connection, n: i64) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    match &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true
        LIMIT
            $1",
        &[&n],
    ) {
        Ok(rows) => {
            for row in rows {
                if let Ok(proxy) = full_from_row(row) {
                    proxies.push(proxy);
                }
            }
        }
        Err(e) => println!("{}", e.to_string()),
    };
    proxies
}

pub fn get_all_work_anon_proxy(conn: Connection) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    if let Ok(rows) = &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true AND anon = true",
        &[],
    ) {
        for row in rows {
            if let Ok(proxy) = full_from_row(row) {
                proxies.push(proxy);
            }
        }
    };
    proxies
}

pub fn get_all_old_proxy(conn: Connection) -> Vec<Proxy> {
    let mut proxies = Vec::new();
    if let Ok(rows) = &conn.query(
        "SELECT
			work, anon, checks, hostname, host, port, scheme, create_at, update_at, response
		FROM
			proxies
		WHERE
			work = true OR update_at < NOW() - (INTERVAL '3 days') * checks",
        &[],
    ) {
        for row in rows {
            if let Ok(proxy) = full_from_row(row) {
                proxies.push(proxy);
            }
        }
    };
    proxies
}
