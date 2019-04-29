use crate::db::Proxy;

impl Proxy {
    fn from(s: &str) -> Result<Self, String> {
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
                raw.get(..pos).ok_or_else(|| format!("not parse scheme {}", raw))?.to_string(),
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
                            raw.get(..pos).ok_or_else(|| format!("not parse host {}", raw))?.to_string(),
                            raw.get(pos + 1..).ok_or_else(|| format!("not parse port {}", raw))?.to_string(),
                        )
                    } else {
                        Err(format!("not parse ipv6 {}", raw))?
                    }
                } else {
                    Err(format!("not parse ipv6 {}", raw))?
                }
            } else {
                (
                    raw.get(..pos).ok_or_else(|| format!("not parse host {}", raw))?.to_string(),
                    raw.get(pos + 1..).ok_or_else(|| format!("not parse port {}", raw))?.to_string(),
                )
            }
        } else {
            Err(format!("not parse port {}", raw))?
        };
        
        let _ = port.parse::<u32>().map_err(|_| format!("not parse port {}", port))?;

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
