use base64::encode;
use percent_encoding::percent_decode_str;
use url::Url;

pub fn base64_auth(url: &Url) -> Option<String> {
    let username = decode(url.username());
    let password = url.password().and_then(|p| decode(p));

    match (username, password) {
        (Some(user), Some(pass)) => Some(encode(&format!("{}:{}", user, pass))),
        _ => None,
    }
}

pub(crate) fn decode(v: &str) -> Option<String> {
    percent_decode_str(v)
        .decode_utf8()
        .map_or(None, |op| Some(op.to_string()))
}
