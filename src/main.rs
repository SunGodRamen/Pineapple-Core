pub mod tcp_interface;
pub mod message_handler;
pub mod file_service;

pub use tcp_interface::server::initialize_server_thread;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    initialize_server_thread();
    Ok(())
}
