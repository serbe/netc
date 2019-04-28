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
) -> Result<CheckedProxy, String> {
    let proxy = reqwest::Proxy::all(proxy_url)
        .map_err(|e| format!("set proxy {} error: {}", proxy_url, e.to_string()))?;
    let client: reqwest::Client = reqwest::Client::builder()
        .proxy(proxy)
        .build()
        .map_err(|e| format!("client builder error {}", e.to_string()))?;
    let body = client
        .get(target_url)
        .send()
        .map_err(|e| format!("get via {} {}", proxy_url, e.to_string()))?
        .text()
        .map_err(|e| format!("convert to text error {}", e.to_string()))?;
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
