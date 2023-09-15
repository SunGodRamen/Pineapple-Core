use std::io::Write;
use std::net::TcpStream;

pub fn send_byte_message(message: &[u8], return_address: &str) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect(return_address)?;
    stream.write_all(message)?;
    Ok(())
}

