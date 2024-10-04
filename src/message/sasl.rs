use rand::Rng;
use stringprep::saslprep;

use crate::core::buf::ExtendWithNull;

use super::{ClientMessage, ClientMsgType};

/// Sasl
/// c: SCRAM-SHA-256\0n,,n=<user>,r=<client_nonce>
/// s: r=<client_nonce>+<server_nonce>,s=<salt>,i=<iterations>
/// c: c=biws,r=<client_nonce>+<server_nonce>,p=<sha-256-hashed-pwd>

pub struct SaslInitialResponse {
    initial_response: String,
}

impl SaslInitialResponse {
    // I'll leave PLUS and its channels until better times
    const MECHANISM: &'static str = "SCRAM-SHA-256";

    pub fn new(username: &str) -> Self {
        let username = saslprep(&username).expect("Failed saslprep for username");
        let nonce: String = random_nonce();

        let initial_response: String = format!("n,,n={username},r={nonce}");

        SaslInitialResponse {
            initial_response,
        }
    }
}

impl ClientMessage for SaslInitialResponse {
    /// Format: p<length><mechanism>\0<scram-length><scram-initial>
    const MSG_TYPE: Option<ClientMsgType> = Some(ClientMsgType::Password);

    fn body_length(&self) -> u32 {
        // additional 4 bytes for client-message-bare length
        // todo: Why didn't you add a null byte after the mechanism (1). I missed something?
        (Self::MECHANISM.len() + self.initial_response.len() + 4) as u32
    }

    fn encode_body(&self, buf: &mut Vec<u8>) {
        buf.extend_with_null(&Self::MECHANISM);
        buf.extend_from_slice(&(self.initial_response.len() as u32).to_be_bytes());
        buf.extend_from_slice(self.initial_response.as_bytes());
    }
}


pub fn random_nonce() -> String {
    let mut rng = rand::thread_rng();
    (0..24)
        .map(|_| rng.sample(rand::distributions::Alphanumeric) as char)
        .collect()
}