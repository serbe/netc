#![allow(dead_code)]
use actix::Message;
use byteorder::{BigEndian, ByteOrder};
use bytes::{BufMut, BytesMut};
use serde_derive::{Deserialize, Serialize};
use serde_json as json;
use std::io;
use tokio_io::codec::{Decoder, Encoder};

/// Client request
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "cmd", content = "data")]
pub enum ClientRequest {
    Message(String),
    Ping,
}

/// Server response
#[derive(Serialize, Deserialize, Debug, Message)]
#[serde(tag = "cmd", content = "data")]
pub enum ClientResponse {
    Ping,
    Message(String),
}

/// Codec for Client -> Server transport
pub struct ClientCodec;

impl Decoder for ClientCodec {
    type Item = ClientRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ClientRequest>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for ClientCodec {
    type Item = ClientResponse;
    type Error = io::Error;

    fn encode(&mut self, msg: ClientResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16_be(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}

/// Codec for Server -> Client transport
pub struct ClientChatCodec;

impl Decoder for ClientChatCodec {
    type Item = ClientResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ClientResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for ClientChatCodec {
    type Item = ClientRequest;
    type Error = io::Error;

    fn encode(&mut self, msg: ClientRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg).unwrap();
        let msg_ref: &[u8] = msg.as_ref();

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16_be(msg_ref.len() as u16);
        dst.put(msg_ref);

        Ok(())
    }
}
