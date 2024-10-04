pub mod startup;
pub mod sasl;


#[repr(u8)]
pub enum ClientMsgType {
    Password = b'p',
}

pub trait ClientMessage {
    const MSG_TYPE: Option<ClientMsgType>;

    fn encode_body(&self, buf: &mut Vec<u8>);

    fn body_length(&self) -> u32;

    fn encode(&self, buf: &mut Vec<u8>) {
        if let Some(msg_type) = Self::MSG_TYPE {
            buf.push(msg_type as u8);
        }

        // type(1) + lenght(4) + body(n)
        let message_length: u32 = 5 + self.body_length();
        buf.extend_from_slice(&message_length.to_be_bytes());

        self.encode_body(buf);
    }
}

// pub trait BackendMessage: Sized {
//     fn decode(buf: &[u8]) -> Result<Self, PgError>;
// }

