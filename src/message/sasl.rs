use bytes::{Bytes, BytesMut};

use crate::{core::buf::PutNull, error::{EncodeError, DecodeError}};
use super::{BackendMessage, FrontendMessage, FrontendFormat};

/// Sasl
/// c: SCRAM-SHA-256\0n,,n=<user>,r=<client_nonce>
/// s: r=<client_nonce>+<server_nonce>,s=<salt>,i=<iterations>
/// c: c=biws,r=<client_nonce>+<server_nonce>,p=<sha-256-hashed-pwd>
///
/// s: SaslAuthentication("R<length>\0SCRAM-SHA-256\0")
/// c: SaslInitialResponse("p<length>SCRAM-SHA-256\0<scram-response-length><scram-response>")
/// s: SaslContinue("R<length>")


#[derive(Debug)]
pub struct AuthenticationSasl(String);

impl BackendMessage for AuthenticationSasl {
    fn decode(buf: &Bytes) -> Result<Self, DecodeError> {
        Ok(
            AuthenticationSasl(String::from_utf8_lossy(buf).to_string())
        )
    }
}


pub struct SaslInitialResponse<'a>(pub &'a str);

impl SaslInitialResponse<'_> {
    // I'll leave PLUS and its channels until better times
    const MECHANISM: &'static str = "SCRAM-SHA-256";
}

impl FrontendMessage for SaslInitialResponse<'_> {
    /// Format: p<length><mechanism>\0<scram-length><scram-initial>
    const FORMAT: Option<FrontendFormat> = Some(FrontendFormat::Password);

    fn body_length(&self) -> u32 {
        // additional 4 bytes for client-message-bare length
        (SaslInitialResponse::MECHANISM.len() + self.0.len() + 4) as u32
    }

    fn encode_body(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.put_null(&SaslInitialResponse::MECHANISM);
        buf.extend_from_slice(&(self.0.len() as u32).to_be_bytes());
        buf.extend_from_slice(self.0.as_bytes());
        Ok(())
    }
}

pub struct SaslContinue {
    pub nonce: String,
    pub salt: String,
    pub iterations: u16,
}

impl BackendMessage for SaslContinue {
    fn decode(buf: &Bytes) -> Result<Self, crate::error::DecodeError> {
        let challange = String::from_utf8_lossy(&buf[4..]);

        let (nonce, salt, iterations) = parse_challange(&challange);

        if nonce.is_empty() || salt.is_empty() {
            return Err(DecodeError("Not found nonce or salt!"));
        }
    
        if iterations < 4096 {
            return Err(DecodeError("Minimum valid iterations is 4096!"));
        }

        Ok(
            SaslContinue {
                nonce,
                salt,
                iterations,
            }
        )
    }
}


fn parse_challange(c: &str) -> (String, String, u16) {
    let mut nonce: String = String::new();
    let mut salt: String = String::new();
    let mut iterations: u16 = 0;

    for s in c.split(',') {
        let mut iter = s.splitn(2, '=');
        let key = iter.next().unwrap();
        let value: &str = iter.next().unwrap();
        match key {
            "r" => {
                nonce = value.to_string();
            },
            "s" => {
                salt = value.to_string();
            },
            "i" => {
                iterations = match value.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
            },
            _ => {}
        }
    }
    
    (nonce, salt, iterations)
}


pub struct SaslResponse<'a>(&'a str);

impl FrontendMessage for SaslResponse<'_> {
    const FORMAT: Option<FrontendFormat> = Some(FrontendFormat::Password);
    
    fn body_length(&self) -> u32 {
        self.0.len() as u32
    }

    fn encode_body(&self, buf: &mut BytesMut) -> Result<(), EncodeError> {
        buf.extend_from_slice(self.0.as_bytes());
        Ok(())
    }
}
