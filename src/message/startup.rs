use bytes::{BufMut, BytesMut};

use crate::{core::buf::PutNull, error::EncodeError};

use super::{FrontendFormat, FrontendMessage};


#[derive(Debug)]
pub struct StartupMessage<'a> {
    parameters: &'a [(&'a str, &'a str)],
}

impl<'a> StartupMessage<'a> {
    const PROTOCOL_VERSION: u32 = 0x00030000; // 3.0

    pub fn new(parameters: &'a [(&'a str, &'a str)]) -> Self {
        StartupMessage {
            parameters,
        }
    }
}

impl FrontendMessage for StartupMessage<'_> {
    /// Format: <length><protocol_version><key1>\0<value1>\0<...>\0\0
    const FORMAT: Option<FrontendFormat> = None;

    fn body_length(&self) -> u32 {
        let mut length: usize = 4;  // protocol_version(4 bytes)

        for (key, value) in self.parameters {
            length += key.len() + value.len() + 2;  // 2 null bytes(1 byte) 
        }

        length as u32 
    }

    fn encode_body<'a>(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.extend_from_slice(&StartupMessage::PROTOCOL_VERSION.to_be_bytes());

        for (key, value) in self.parameters {
            buf.put_null(&key);
            buf.put_null(&value);
        }
        buf.put_u8(0);

        Ok(())
    }
}
