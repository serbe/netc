#[derive(Debug)]
pub struct CheckedProxy {
    pub hostname: String,
    pub work: bool,
    pub anon: bool,
}

pub fn my_ip() -> Result<String, reqwest::Error> {
    reqwest::get("https://api.ipify.org")?.text()
}

pub fn get_checked_proxy(
    proxy_url: &str,
    target_url: &str,
    my_ip: &str,
) -> Result<CheckedProxy, reqwest::Error> {
    let proxy = reqwest::Proxy::all(proxy_url)?;
    let client: reqwest::Client = reqwest::Client::builder().proxy(proxy).build()?;
    let body = client.get(target_url).send()?.text()?;
    if !body.contains(my_ip) {
        if body.matches("<p>").count() == 1 {
            Ok(CheckedProxy {
                hostname: proxy_url.to_string(),
                work: true,
                anon: true,
            })
        } else {
            Ok(CheckedProxy {
                hostname: proxy_url.to_string(),
                work: true,
                anon: false,
            })
        }
    } else {
        Ok(CheckedProxy {
            hostname: proxy_url.to_string(),
            work: false,
            anon: false,
        })
    }
}
