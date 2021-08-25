// use uri::Uri;

// use crate::Error;

// pub trait IntoUri: IntoUriSealed {}

// impl IntoUri for Uri {}
// impl IntoUri for &Uri {}
// impl IntoUri for String {}
// impl IntoUri for &String {}
// impl<'a> IntoUri for &'a str {}

// pub trait IntoUriSealed {
//     fn into_uri(self) -> Result<Uri, Error>;

//     fn as_str(&self) -> &str;
// }

// impl IntoUriSealed for Uri {
//     fn into_uri(self) -> Result<Uri, Error> {
//         if self.has_host() {
//             Ok(self)
//         } else {
//             Err(Error::EmptyHost)
//         }
//     }

//     fn as_str(&self) -> &str {
//         self.as_ref()
//     }
// }

// impl IntoUriSealed for &Uri {
//     fn into_uri(self) -> Result<Uri, Error> {
//         if self.has_host() {
//             Ok(self.clone())
//         } else {
//             Err(Error::EmptyHost)
//         }
//     }

//     fn as_str(&self) -> &str {
//         self.as_ref()
//     }
// }

// impl<'a> IntoUriSealed for &'a str {
//     fn into_uri(self) -> Result<Uri, Error> {
//         Uri::parse(self)?.into_uri()
//     }

//     fn as_str(&self) -> &str {
//         self
//     }
// }

// impl IntoUriSealed for &String {
//     fn into_uri(self) -> Result<Uri, Error> {
//         (&**self).into_uri()
//     }

//     fn as_str(&self) -> &str {
//         self.as_ref()
//     }
// }

// impl<'a> IntoUriSealed for String {
//     fn into_uri(self) -> Result<Uri, Error> {
//         (&*self).into_uri()
//     }

//     fn as_str(&self) -> &str {
//         self.as_ref()
//     }
// }
