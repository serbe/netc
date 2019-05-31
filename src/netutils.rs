use crate::db::Proxy;
use chrono::Local;
use std::time::Instant;

pub fn my_ip() -> Result<String, reqwest::Error> {
    reqwest::get("https://api.ipify.org")?.text()
}

pub fn check_proxy(proxy_url: &str, target_url: &str, my_ip: &str) -> Result<Proxy, String> {
    let dur = Instant::now();
    let proxy = reqwest::Proxy::all(proxy_url)
        .map_err(|e| format!("set proxy {} error: {}", proxy_url, e.to_string()))?;
    let client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .map_err(|e| format!("client builder error {}", e.to_string()))?;
    let body = client
        .get(target_url)
        .send()
        .map_err(|e| format!("get via {} {}", proxy_url, e.to_string()))?
        .text()
        .map_err(|e| format!("convert to text error {}", e.to_string()))?;
    let mut proxy = Proxy::from(proxy_url)?;
    proxy.work = true;
    if !body.contains(my_ip) && body.matches("<p>").count() == 1 {
        proxy.anon = true;
    }
    proxy.create_at = Local::now();
    proxy.update_at = Local::now();
    proxy.response = dur.elapsed().as_micros() as i64;
    Ok(proxy)
}
