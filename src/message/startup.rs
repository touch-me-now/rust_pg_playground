use crate::core::buf::ExtendWithNull;

use super::{ClientMessage, ClientMsgType};


#[derive(Debug)]
pub struct StartupMessage<'a> {
    pub protocol_version: u32,
    parameters: &'a [(&'a str, &'a str)],
}

impl<'a> StartupMessage<'a> {
    pub fn new(parameters: &'a [(&'a str, &'a str)]) -> Self {
        StartupMessage {
            protocol_version: 0x00030000, // 3.0
            parameters,
        }
    }
}

impl ClientMessage for StartupMessage<'_> {
    /// Format: <length><protocol_version><key1>\0<value1>\0<...>\0\0
    const MSG_TYPE: Option<ClientMsgType> = None;

    fn body_length(&self) -> u32 {
        let mut length: usize = 4;  // protocol_version(4 bytes)

        for (key, value) in self.parameters {
            length += key.len() + value.len() + 2;  // 2 null bytes(1 byte) 
        }

        length as u32 
    }

    fn encode_body<'a>(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.protocol_version.to_be_bytes());

        for (key, value) in self.parameters {
            buf.extend_with_null(key);
            buf.extend_with_null(value);
        }
        buf.push(0);
    }
}
