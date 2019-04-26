fn my_ip() -> Result<String, reqwest::Error> {
    reqwest::get("https://api.ipify.org")?.text()
}

fn check_proxy(proxy_url: &str, target_url: &str, my_ip: &str) -> Result<String, reqwest::Error> {
    let proxy = reqwest::Proxy::all(proxy_url)?;
    let client: reqwest::Client = reqwest::Client::builder().proxy(proxy).build()?;
    client.get(target_url).send()?.text()
}
