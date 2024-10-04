mod core;
mod message;
mod error;

use std::{io::{Read, Write}, net::TcpStream};

use crate::message::{ClientMessage, startup::StartupMessage, sasl::SaslInitialResponse};


const BUF_SIZE: usize = 8192;

struct PgMessage<'a>(&'a [u8]);

impl<'a> PgMessage<'a> {
    pub fn from_buf(buf: &'a Vec<u8>) -> Self {
        PgMessage(buf)
    }

    pub fn msg_type(&self) -> char {
        self.0[0] as char
    }

    fn length_bytes(&self) -> &[u8] {
        &self.0[1..5]
    }

    pub fn length(&self) -> u32 {
        u32::from_be_bytes(self.length_bytes().try_into().unwrap())
    }

    fn body_bytes(&self) -> &[u8] {
        &self.0[5..(self.length() as usize)]
    }

    pub fn body_string(&self) -> String {
        String::from_utf8_lossy(&self.body_bytes()).to_string()
    }

    pub fn get_auth_type(&self) -> u32 {
        assert!(self.0.len() >= 8);

        u32::from_be_bytes(self.0[5..9].try_into().unwrap())
    }
}


fn main() {
    let mut stream = TcpStream::connect("localhost:5432").unwrap();
    let mut buffer = vec![0u8; BUF_SIZE];

    let parameters = vec![("user", "postgres"), ("database", "postgres"), ("client_encoding", "UTF8")];
    
    println!("Startup...\n");
    let mut startup_buf = Vec::new();

    StartupMessage::new(&parameters).encode(&mut startup_buf);
    println!("{:#?}", String::from_utf8_lossy(&startup_buf.clone()));

    stream.write_all(&startup_buf).expect("Failed to send startup message");

    let n = stream.read(&mut buffer).expect("Failed to read from stream");
    if n > 0 {
        let msg = PgMessage::from_buf(&buffer);

        println!("Message type: {}", msg.msg_type());
        println!("Lenght: {}", msg.length());
        println!("Body: {:?}", msg.body_string());

        match msg.msg_type() {
            'R' => {
                println!("Auth...\n");
                match msg.get_auth_type() {
                    10 => {
                        println!("Sasl...\n");

                        
                        let mut initial_buf = Vec::new();

                        SaslInitialResponse::new("postgres")
                            .encode(&mut initial_buf);

                        println!("{:?}", String::from_utf8_lossy(&initial_buf.clone()));

                        stream.write_all(&initial_buf).expect("Failed to send mechanism");

                        let mut response = [0; 4096];
                        let n = stream.read(&mut response).expect("Failed to read response");

                        let msg = String::from_utf8_lossy(&response[..n]);
                        println!("Received: {:?}", msg);
                    },
                    ot => {
                        println!("Unsupported auth type: {}", ot);
                    }
                }
            },
            'E' => {
                // 'E' означает сообщение ошибки
                let error_message = String::from_utf8_lossy(&buffer[5..(msg.length() as usize)]);
                println!("Error Message: {}", error_message);
            },
            other => {
                println!("Unknown message type received: {}", other);
            }
        }
    }

    


    
    // let n = stream.read(&mut buffer).expect("Failed to read from stream");

    // if n > 0 {
    //     let msg_type = buffer[0] as char;

    //     let length_bytes = &buffer[1..5];
    //     let length: u32 = u32::from_be_bytes(length_bytes.try_into().unwrap());

    //     let body_bytes = &buffer[5..(length as usize)];
    //     let body = String::from_utf8_lossy(body_bytes).to_string();

    //     println!("body: {}", body);

    //     match msg_type {
    //         'R' => {
    //             println!("Auth: {}", String::from_utf8_lossy(&buffer[5..9]));

    //             let auth_type = u32::from_be_bytes(buffer[5..9].try_into().unwrap());
    //             println!("Authentication Type: {}", auth_type);

    //             match auth_type {
    //                 10 => {
    //                     println!("auth type sasl");
    //                     let sasl_initial = SaslInitialResponse::new("postgres").encode().unwrap();
    //                     // println!("sasl initial: {:?}", String::from_utf8(sasl_initial));

    //                     stream.write_all(&sasl_initial).expect("Failed to send sasl initial response message");

    //                 },
    //                 _ => {
    //                     println!("Unsupported auth type: {}", auth_type);
    //                 }
    //             }

    //             // todo: нужно реализовать Authentication
    //         },
    //         'E' => {
    //             // 'E' означает сообщение ошибки
    //             let error_message = String::from_utf8_lossy(&buffer[5..(length as usize)]);
    //             println!("Error Message: {}", error_message);
    //         },
    //         _ => {
    //             println!("Unknown message type received: {}", msg_type);
    //         }
    //     }

    // }
    // let n = stream.read(buffer)?;
    
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}
