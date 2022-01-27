// use base64::encode;
use url::Url;
// use percent_encoding::percent_decode_str;

use crate::Error;

pub(crate) fn relative_quality_factor<T: ToString + ?Sized>(value: &T) -> Option<f32> {
    let value = value.to_string();
    if value.is_empty() {
        return None;
    };
    value
        .split(';')
        .nth(1)
        .and_then(|v| v.split("q=").nth(1))
        .and_then(|v| v.parse().ok())
        .or(Some(1.0f32))
}

pub(crate) fn array_from_string<T: ToString>(value: T) -> Vec<String> {
    value
        .to_string()
        .split(',')
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .collect()
}

pub trait IntoUrl {
    fn into_url(self) -> Result<Url, Error>;
}

// impl IntoUrl for Url {
//     fn into_uri(self) -> Result<Uri, Error> {
//         if self.has_authority() {
//             Ok(self)
//         } else {
//             Err(Error::EmptyAuthority)
//         }
//     }
// }

impl<'a> IntoUrl for &'a Url {
    fn into_url(self) -> Result<Url, Error> {
        Ok(self.clone())
    }
}

impl<'a> IntoUrl for &'a str {
    fn into_url(self) -> Result<Url, Error> {
        Ok(Url::parse(self)?)
    }
}

impl<'a> IntoUrl for &'a String {
    fn into_url(self) -> Result<Url, Error> {
        Ok(Url::parse(&**self)?)
    }
}

// impl<'a> IntoUrl for String {
//     fn into_url(self) -> Result<Url, Error> {
//         Ok(Url::parse(&*self)?)
//     }
// }

// pub fn into_url<U: IntoUrl>(u: U) -> Result<Url, Error> {
//     match u.into_url() {
//         Ok(url) => Ok(url),
//         Err(err) => Err(err),
//     }
// }

pub(crate) fn host_header(url: &Url) -> String {
    match (url.host_str(), url.port()) {
        (Some(host), Some(port)) => format!("{}:{}", host, port),
        (Some(host), None) => format!("{}", host),
        _ => String::new(),
    }
}

pub(crate) fn request_uri(url: &Url, proxy: bool) -> String {
    if proxy {
        absolute_uri(url).to_string()
    } else {
        abs_path(url)
    }
}

pub(crate) fn abs_path(url: &Url) -> String {
    let mut path = url.path().to_string();
    if let Some(query) = url.query() {
        path.push('?');
        path.push_str(query)
    };
    if let Some(fragment) = url.fragment() {
        path.push('#');
        path.push_str(fragment)
    };
    path
}

pub(crate) fn absolute_uri(url: &Url) -> &str {
    url.as_str()
}

// pub(crate) fn decode(v: Option<&str>) -> Option<String> {
//     v.filter(|s| !s.is_empty()).map(|s| percent_decode_str(s).decode_utf8().ok()).flatten().map(|v| v.to_string())
// }

// pub(crate) fn base64_auth(url: &Url) -> Option<String> {
//     if url.scheme() == "http" || url.scheme() == "https" {
//         match (decode(Some(url.username())), decode(url.password())) {
//             (Some(user), Some(pass)) => Some(encode(&format!("{}:{}", user, pass))),
//             _ => None,
//         }
//     } else {
//         None
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_intourl() {
        assert!("http://127.0.0.1:1010".into_url().is_ok())
    }

    #[test]
    fn quality_10_1() {
        assert_eq!(relative_quality_factor("text/html"), Some(1.0f32));
    }

    #[test]
    fn quality_10_2() {
        assert_eq!(relative_quality_factor("text/html; q=1"), Some(1.0f32));
    }

    #[test]
    fn quality_10_3() {
        assert_eq!(relative_quality_factor("text/html; q=asd"), Some(1.0f32));
    }

    #[test]
    fn quality_07_1() {
        assert_eq!(relative_quality_factor("text/html; q=0.7"), Some(0.7f32));
    }

    #[test]
    fn quality_07_2() {
        assert_eq!(relative_quality_factor("text/html;q=0.7"), Some(0.7f32));
    }

    #[test]
    fn string2array_1() {
        assert!(array_from_string("").is_empty());
    }

    #[test]
    fn string2array_2() {
        assert_eq!(array_from_string("text/html;q=0.7").len(), 1);
    }

    #[test]
    fn string2array_3() {
        assert_eq!(array_from_string("text/html;q=0.7,,text/x-dvi").len(), 2);
    }

    #[test]
    fn string2array_4() {
        assert_eq!(
            array_from_string("text/html;q=0.7,,text/x-dvi"),
            vec!["text/html;q=0.7".to_string(), "text/x-dvi".to_string()]
        );
    }
}
