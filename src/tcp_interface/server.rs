use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::thread;

use crate::message_handler::router;
use crate::message_handler::MAX_MSG_BYTE_LENGTH;

pub fn read_byte_message(mut stream: TcpStream) {
    let mut buffer = [0; MAX_MSG_BYTE_LENGTH];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Error reading from stream: {}", e);
        return;
    }    

    // pass the read content to the router to handle
    router::handle_message(&buffer);
}

pub fn initialize_server_thread() {
    let listener = TcpListener::bind("127.0.0.1:4040").expect("Failed to bind to address"); //**configuration for port instead

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    read_byte_message(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
