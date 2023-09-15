pub mod router;
pub mod interpreter;

pub const MAX_MSG_BYTE_LENGTH: usize = 1024;

pub use router::handle_message;

pub use interpreter::{decode_request, encode_response, RequestMessage, ResponseMessage};