use core::ToBytes;
use std::{io::{Read, Write}, net::TcpStream};


mod core {
    pub mod buffer {
        pub trait Buf {
            fn extend_with_nil(&mut self, other: &[u8]);
        }
        
        
        impl Buf for Vec<u8> {
            fn extend_with_nil(&mut self, other: &[u8]) {
                self.extend_from_slice(other);
                self.push(0);
            }
        }
    }
    
    pub trait ToBytes {
        fn to_bytes(&self) -> Vec<u8>;
    }
    
    pub trait CalcLength {
        fn calculate_length(&self, body: &[u8]) -> u32 {
            (8 + body.len()) as u32
        }
    }
    
}

mod messages {
    pub mod startup {
        use crate::core::{buffer::Buf, CalcLength, ToBytes};

        #[derive(Debug)]
        pub struct StartupMessage<'a> {
            pub protocol_version: u32,
            parameters: &'a [(&'a str, &'a str)],
        }
    
        impl<'a> StartupMessage<'a> {
            pub fn new(parameters: &'a [(&'a str, &'a str)]) -> Self {
                // версию протокола я пока буду устанавливать здесь
                // чтобы сосредаточится на одной версии протокола
                // todo: на будующее надо бы узнать какие изменения произходили между протоколами
                StartupMessage {
                    protocol_version: 0x00030000,
                    parameters
                }
            }

            fn body_bytes(&self) -> Vec<u8> {
                let mut body: Vec<u8> = Vec::new();
        
                for (key, value) in self.parameters {
                    body.extend_with_nil(key.as_bytes());
                    body.extend_with_nil(value.as_bytes());
                }
                body.push(0);
                body
            }
        }
    
        impl CalcLength for StartupMessage<'_> {}
    
        impl ToBytes for StartupMessage<'_> {
            fn to_bytes(&self) -> Vec<u8> {
                let body: Vec<u8> = self.body_bytes();
                let message_length: u32 = self.calculate_length(&body);
        
                let mut buffer: Vec<u8> = Vec::new();
                buffer.extend_from_slice(&message_length.to_be_bytes());
                buffer.extend_from_slice(&self.protocol_version.to_be_bytes());

                // buffer.extend_with_nil(&body);
                buffer.extend_from_slice(&body);
                buffer
            }
        }
    }
}

use messages::startup::StartupMessage;

fn main() {
    let mut stream = TcpStream::connect("localhost:5432").unwrap();

    let parameters = vec![("user", "postgres"), ("database", "test"), ("client_encoding", "UTF8")];
    let startup_msg: Vec<u8> = StartupMessage::new(&parameters).to_bytes();
    stream.write_all(&startup_msg).expect("Failed to send startup message");

    let mut buffer = [0; 512];  // буфер для чтения
    let n = stream.read(&mut buffer).expect("Failed to read from stream");

    if n > 0 {
        // Читаем тип сообщения
        let message_type = buffer[0] as char;
        println!("Message Type: {}", message_type);

        let response_str = String::from_utf8_lossy(&buffer[..n]);

        println!("PSQL Response: {:?}", response_str);

        
        // Читаем длину сообщения
        let message_length = u32::from_be_bytes([buffer[1], buffer[2], buffer[3], buffer[4]]);
        println!("Message Length: {}", message_length);

        // Выводим содержимое сообщения (пример, как читать разные типы сообщений)
        match message_type {
            'R' => {
                // 'R' означает сообщение аутентификации
                
                let auth_type = u32::from_be_bytes([buffer[5], buffer[6], buffer[7], buffer[8]]);
                println!("Authentication Type: {}", auth_type);

                // todo: нужно реализовать Authentication
            },
            'E' => {
                // 'E' означает сообщение ошибки
                let error_message = String::from_utf8_lossy(&buffer[5..(message_length as usize)]);
                println!("Error Message: {}", error_message);
            },
            _ => {
                println!("Unknown message type received: {}", message_type);
            }
        }
    }

    stream.shutdown(std::net::Shutdown::Both).unwrap();
}