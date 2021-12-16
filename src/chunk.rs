use tokio::io::AsyncReadExt;

use crate::Error;

// length := 0
// read chunk-size, chunk-ext (if any), and CRLF
// while (chunk-size > 0) {
//    read chunk-data and CRLF
//    append chunk-data to decoded-body
//    length := length + chunk-size
//    read chunk-size, chunk-ext (if any), and CRLF
// }
// read trailer field
// while (trailer field is not empty) {
//    if (trailer field is allowed to be sent in a trailer) {
//       append trailer field to existing header fields
//    }
//    read trailer-field
// }
// Content-Length := length
// Remove "chunked" from Transfer-Encoding
// Remove Trailer from existing header fields

async fn read_chunk_size(body: &[u8]) -> Result<usize, Error> {
    let mut body = body.clone();
    let mut size_line = Vec::new();
    let mut ext = false;  
    loop {
        match body.read_u8().await? {
            value if value == b';' => {ext = true},
            value if value == b'\r' => break,
            value if !ext => size_line.push(value),
            _ => (),
        }
    }
    let size = match body.read_u8().await? {
        value if value != b'\n' => return Err(Error::InvalidChunkSize),
        _ =>  usize::from_str_radix(String::from_utf8(size_line)?.trim(), 16)?,
    };
    Ok(size)
}

// pub struct Decoder<R> {
//     source: R,
//     remaining_chunks_size: Option<usize>,
// }

// impl<R> Decoder<R>
// where
//     R: Read,
// {
//     pub fn new(source: R) -> Decoder<R> {
//         Decoder {
//             source,
//             remaining_chunks_size: None,
//         }
//     }

//     pub fn remaining_chunks_size(&self) -> Option<usize> {
//         self.remaining_chunks_size
//     }

//     pub fn into_inner(self) -> R {
//         self.source
//     }

//     fn read_chunk_size(&mut self) -> IoResult<usize> {
//         let mut chunk_size_bytes = Vec::new();
//         let mut has_ext = false;

//         loop {
//             let byte = match self.source.by_ref().bytes().next() {
//                 Some(b) => b?,
//                 None => return Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
//             };

//             if byte == b'\r' {
//                 break;
//             }

//             if byte == b';' {
//                 has_ext = true;
//                 break;
//             }

//             chunk_size_bytes.push(byte);
//         }

//         if has_ext {
//             loop {
//                 let byte = match self.source.by_ref().bytes().next() {
//                     Some(b) => b?,
//                     None => return Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
//                 };
//                 if byte == b'\r' {
//                     break;
//                 }
//             }
//         }

//         self.read_line_feed()?;

//         let chunk_size = String::from_utf8(chunk_size_bytes)
//             .ok()
//             .and_then(|c| usize::from_str_radix(c.trim(), 16).ok())
//             .ok_or_else(|| IoError::new(ErrorKind::InvalidInput, DecoderError))?;

//         Ok(chunk_size)
//     }

//     fn read_carriage_return(&mut self) -> IoResult<()> {
//         match self.source.by_ref().bytes().next() {
//             Some(Ok(b'\r')) => Ok(()),
//             _ => Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
//         }
//     }

//     fn read_line_feed(&mut self) -> IoResult<()> {
//         match self.source.by_ref().bytes().next() {
//             Some(Ok(b'\n')) => Ok(()),
//             _ => Err(IoError::new(ErrorKind::InvalidInput, DecoderError)),
//         }
//     }
// }

// impl<R> Read for Decoder<R>
// where
//     R: Read,
// {
//     fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
//         let remaining_chunks_size = match self.remaining_chunks_size {
//             Some(c) => c,
//             None => {
//                 // first possibility: we are not in a chunk, so we'll attempt to determine
//                 // the chunks size
//                 let chunk_size = self.read_chunk_size()?;

//                 // if the chunk size is 0, we are at EOF
//                 if chunk_size == 0 {
//                     self.read_carriage_return()?;
//                     self.read_line_feed()?;
//                     return Ok(0);
//                 }

//                 chunk_size
//             }
//         };

//         // second possibility: we continue reading from a chunk
//         if buf.len() < remaining_chunks_size {
//             let read = self.source.read(buf)?;
//             self.remaining_chunks_size = Some(remaining_chunks_size - read);
//             return Ok(read);
//         }

//         // third possibility: the read request goes further than the current chunk
//         // we simply read until the end of the chunk and return
//         assert!(buf.len() >= remaining_chunks_size);

//         let buf = &mut buf[..remaining_chunks_size];
//         let read = self.source.read(buf)?;

//         self.remaining_chunks_size = if read == remaining_chunks_size {
//             self.read_carriage_return()?;
//             self.read_line_feed()?;
//             None
//         } else {
//             Some(remaining_chunks_size - read)
//         };

//         Ok(read)
//     }
// }