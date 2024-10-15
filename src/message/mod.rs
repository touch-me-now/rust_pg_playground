pub mod sasl;
pub mod startup;

use bytes::{BufMut, Bytes, BytesMut};
    
use crate::error::{EncodeError, DecodeError};

#[repr(u8)]
pub enum FrontendFormat {
    Password = b'p',
}

pub trait FrontendMessage {
    const FORMAT: Option<FrontendFormat>;
    
    fn encode_body(&self, buf: &mut BytesMut) -> Result<(), EncodeError>;

    fn body_length(&self) -> u32;

    fn length(&self) -> u32 {
        // type(1) + lenght(4) + body(n)
        5 + self.body_length()
    }

    fn encode(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        if let Some(msg_type) = Self::FORMAT {
            buf.put_u8(msg_type as u8);
        }
        
        buf.extend_from_slice(&self.length().to_be_bytes());
        self.encode_body(buf)
    }
}

pub trait BackendMessage: Sized {
    fn decode(buf: &Bytes) -> Result<Self, DecodeError>;
}

pub struct PgMessage {
    pub msg_type: char,
    pub content: Bytes
}


impl BackendMessage for PgMessage {
    fn decode(buf: &Bytes) -> Result<Self, DecodeError> {
        Ok(
            PgMessage { 
                msg_type: buf[0] as char, 
                content: buf.slice(5..),
            }
        )
    }
}