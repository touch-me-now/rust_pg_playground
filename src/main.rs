mod core;
mod error;
mod message;
mod stream;

use std::error::Error;

use crate::message::startup::StartupMessage;
use crate::message::sasl::AuthenticationSasl;

use crate::stream::PgStream;

// fn simple() {
    // const BUF_SIZE: usize = 8192;
//     let mut stream = TcpStream::connect("localhost:5432").unwrap();
//     let mut read_buf = vec![0u8; BUF_SIZE];
//     let mut write_buf: Vec<u8> = Vec::new();

//     let parameters = vec![("user", "postgres"), ("database", "postgres"), ("client_encoding", "UTF8")];
    
//     println!("Startup...\n");
//     let startup = StartupMessage::new(&parameters);
//     startup.encode(&mut write_buf).unwrap();

//     println!("{:#?}", String::from_utf8_lossy(&write_buf.clone()));
//     stream.write_all(&write_buf).expect("Failed to send startup message");

//     let n = stream.read(&mut read_buf).expect("Failed to read from stream");
//     if n > 0 {
//         let pg_msg = PgMessage::decode(&read_buf).unwrap();

//         println!("Message type: {}", pg_msg.msg_type);
//         println!("Lenght: {}", pg_msg.length);
//         println!("Body: {:?}", String::from_utf8_lossy(&pg_msg.content));

//         match pg_msg.msg_type {
//             'R' => {
//                 println!("Auth...\n");
//                 let auth_type = u32::from_be_bytes(read_buf[5..9].try_into().unwrap());

//                 match auth_type {
//                     10 => {
//                         println!("Sasl...\n");
//                         write_buf.clear();

//                         let username = saslprep("postgres").unwrap();
//                         let nonce = random_nonce();

//                         let cli_msg_bare = format!("n,,n={},r={}", username, nonce);
//                         let sasl_init_resp = SaslInitialResponse(&cli_msg_bare);
                        
//                         sasl_init_resp.encode(&mut write_buf).unwrap();
//                         println!("{:?}", String::from_utf8_lossy(&write_buf.clone()));

//                         stream.write_all(&write_buf).expect("Failed to send mechanism");

//                         stream.read(&mut read_buf).expect("Failed to read response");

//                         let con = PgMessage::decode(&read_buf).unwrap();
//                         let body: Vec<u8> = con.content.to_vec();

//                         let sasl_con = SaslContinue::decode(&body).unwrap();
//                         println!("nonce: {} salt: {} i: {}", sasl_con.nonce, sasl_con.salt, sasl_con.iterations);
//                         // let body = con.body.to_vec();


//                         println!("Received: {:?}", String::from_utf8_lossy(&con.content));
//                     },
//                     ot => {
//                         println!("Unsupported auth type: {}", ot);
//                     }
//                 }
//             },
//             'E' => {
//                 // 'E' означает сообщение ошибки
//                 let error_message = String::from_utf8_lossy(&&read_buf[5..(pg_msg.length as usize)]);
//                 println!("Error Message: {}", error_message);
//             },
//             other => {
//                 println!("Unknown message type received: {}", other);
//             }
//         }
//     }

//     stream.shutdown(std::net::Shutdown::Both).unwrap();
// }

fn main() -> Result<(), Box<dyn Error>> {
    // simple();

    let mut stream = PgStream::connect("localhost:5432")?;

    
    println!("Startup...\n");
    let parameters = vec![("user", "postgres"), ("database", "postgres"), ("client_encoding", "UTF8")];
    stream.send(
        StartupMessage::new(&parameters)
    )?;
    let auth_response: AuthenticationSasl = stream.expect()?;

    println!("{:?}", auth_response);
    Ok(())
}
