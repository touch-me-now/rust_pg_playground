use std::io::{self, Read, Write};
use std::net::TcpStream;

use bytes::{Bytes, BytesMut};

use crate::error::PgError;
use crate::message::{FrontendMessage, BackendMessage, PgMessage};


pub struct PgStream {
    stream: TcpStream,
    read_buf: BytesMut,
    write_buf: BytesMut
}

const DEFAULT_BUF_SIZE: usize = 1024;

impl PgStream {
    pub fn connect(addr: &str) -> Result<Self, std::io::Error> {
        Ok(
            PgStream {
                stream: TcpStream::connect(addr)?,
                read_buf: BytesMut::with_capacity(DEFAULT_BUF_SIZE),
                write_buf: BytesMut::with_capacity(DEFAULT_BUF_SIZE),
            }
        )
    }
    
    pub fn send(&mut self, msg: impl FrontendMessage) -> Result<(), PgError> {
        msg.encode(&mut self.write_buf)?;
        println!("{:#?}", self.write_buf);

        self.stream.write_all(&self.write_buf).map_err(|err| PgError::IOError(err))?;
        self.write_buf.clear();
        Ok(())
    }

    fn read(&mut self) -> io::Result<Bytes> {
        let mut headers = [0u8; 5];

        self.stream.read_exact(&mut headers)?;

        let message_length = u32::from_be_bytes(headers[1..].try_into().unwrap()) as usize;

        self.read_buf.extend_from_slice(&headers);
        self.read_buf.resize(message_length, 0);
        
        self.stream.read_exact(&mut self.read_buf[5..])?;

        let frozen_buf = self.read_buf.split().freeze();

        Ok(frozen_buf)
    }

    pub fn expect<T: BackendMessage>(&mut self) -> Result<T, PgError> {
        let bytes = self.read().map_err(|err| PgError::IOError(err))?;

        let pg_msg = PgMessage::decode(&bytes)?;
        match pg_msg.msg_type {
            'E' => {
                let error = String::from_utf8_lossy(&pg_msg.content);
                return Err(PgError::ErrorResp(error.to_string()));
            },
            _ => {
                return Ok(T::decode(&pg_msg.content)?);
            }
        }
    }
    
}