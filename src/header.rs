// Header Fields
//
//    Each header field consists of a case-insensitive field name followed
//    by a colon (":"), optional leading whitespace, the field value, and
//    optional trailing whitespace.
//
//
// RFC 7230           HTTP/1.1 Message Syntax and Routing         June 2014
//
//      header-field   = field-name ":" OWS field-value OWS
//
//      field-name     = token
//      field-value    = *( field-content / obs-fold )
//      field-content  = field-vchar [ 1*( SP / HTAB ) field-vchar ]
//      field-vchar    = VCHAR / obs-text
//
//      obs-fold       = CRLF 1*( SP / HTAB )
//                     ; obsolete line folding
//                     ; see Section 3.2.4
//
//    The field-name token labels the corresponding field-value as having
//    the semantics defined by that header field.  For example, the Date
//    header field is defined in Section 7.1.1.2 of [RFC7231] as containing
//    the origination timestamp for the message in which it appears.
// #[warn(soft_unstable)]
use std::collections::HashMap;
use std::str::FromStr;

use crate::utils::trim;
use crate::Error;
use bytes::Bytes;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderFields {
    pub(crate) name: Bytes,
    pub(crate) value: Bytes,
}

#[derive(Clone, Debug)]
pub struct Headers(HashMap<Bytes, HeaderFields>);

impl HeaderFields {
    pub fn new<T: Into<Bytes>, U: Into<Bytes>>(name: T, value: U) -> Result<Self, Error> {
        let name: Bytes = trim(&name.into());
        let value = value.into();
        if !name[0].is_ascii_uppercase() && name[0] != b'*' {
            return Err(Error::HeaderWrongNameStart);
        }
        if !name
            .split(|c| c == &b'-')
            .all(|word| word.iter().all(|c| c.is_ascii_alphanumeric()))
        {
            return Err(Error::HeaderWrongName);
        }

        Ok(HeaderFields { name, value })
    }
}

impl FromStr for HeaderFields {
    type Err = Error;

    fn from_str(str_line: &str) -> Result<HeaderFields, Error> {
        let b = Bytes::from(str_line.to_owned());
        let mut sp = b.splitn(2, |pred| pred == &b':');
        if let (Some(name), Some(mut value)) = (sp.next(), sp.next()) {
            value = &value[1..];
            if value[0].is_ascii_whitespace() && value.len() > 1 {
                value = &value[1..];
            }
            HeaderFields::new(name.to_owned(), value.to_owned())
        } else {
            Err(Error::ParseHeaders)
        }
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

impl Headers {
    pub fn new() -> Self {
        Headers(HashMap::new())
    }

    pub fn insert<T: Into<Bytes> + Clone, U: Into<Bytes>>(
        &mut self,
        name: T,
        value: U,
    ) -> Result<Option<HeaderFields>, Error> {
        Ok(self.0.insert(
            name.clone().into().to_ascii_uppercase().into(),
            HeaderFields::new(name, value)?,
        ))
    }

    pub fn get<T: Into<Bytes> + Clone>(&self, name: T) -> Option<&HeaderFields> {
        let name: Bytes = name.into().to_ascii_uppercase().into();
        self.0.get(&name)
    }

    pub fn remove<T: Into<Bytes> + Clone>(&mut self, name: T) -> Option<HeaderFields> {
        let name: Bytes = name.into().to_ascii_uppercase().into();
        self.0.remove(&name)
    }

    pub fn get_quality_factor<T: Into<Bytes> + Clone>(&self, name: T) -> Option<f32> {
        let name: Bytes = name.into().to_ascii_uppercase().into();
        self.0.get(&name).and_then(header_relative_quality_factor)
    }

    // pub fn get_array<T: ToString + ?Sized>(&self, key: &T) -> Vec<String> {
    //     self.get(key).map_or_else(Vec::new, array_from_string)
    // }

    pub fn content_length(&self) -> Option<usize> {
        self.get("Content-Length")
            .and_then(|v| String::from_utf8_lossy(&v.value).parse().ok())
    }
}

fn header_relative_quality_factor(header: &HeaderFields) -> Option<f32> {
    if header.value.is_empty() {
        return None;
    };
    let value = String::from_utf8_lossy(&header.value);
    value
        .split(';')
        .nth(1)
        .and_then(|v| v.split("q=").nth(1))
        .and_then(|v| v.parse().ok())
        .or(Some(1.0f32))
}

#[cfg(test)]
mod tests {
    // use bytes::BytesMut;

    // #[test]
    // fn capitalize_1() {
    //     let mut header = BytesMut::from("UsEr-aGeNt");
    //     capitalize(&mut header);
    //     assert_eq!(header, BytesMut::from("User-Agent"));
    // }
}
