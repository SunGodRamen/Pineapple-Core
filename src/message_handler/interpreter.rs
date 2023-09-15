// Pineapple Message Request Format:
// Strings are ascii encoded as bytes
// The first 4 bytes are the id
// The next bytes are the return address until the first null byte
// The next bytes are the resource indicator until the second null byte
// The next bytes are the body until the third null byte
pub struct RequestMessage {
    pub id: u32,
    pub return_address: String,
    pub resource_indicator: String,
    pub body: String,
}

// Pineapple Message Confirmation Format:
// Strings are ascii encoded as bytes
// The first 4 bytes are the id
// The next 4 bytes are the status code
// The next bytes are the return address until the first null byte
pub struct ConfirmationMessage {
    pub id: u32,
    pub status_code: u32,
    pub return_address: String,
}


// Pineapple Message Response Format:
// Strings are ascii encoded as bytes
// The first 4 bytes are the id
// The next 4 bytes are the status code
// The next bytes are the return address until the first null byte
// The next bytes are the body until the second null bytew
pub struct ResponseMessage {
  pub id: u32,
  pub status_code: u32,
  pub return_address: String,
  pub body: String,
}

pub fn decode_request(message_buffer: &[u8]) -> RequestMessage {
  let id = u32::from_be_bytes([message_buffer[0], message_buffer[1], message_buffer[2], message_buffer[3]]);
  let parts: Vec<&[u8]> = message_buffer[4..].split(|&b| b == 0).collect();
  
  let return_address = String::from_utf8_lossy(parts[0]).to_string();
  let resource_indicator = String::from_utf8_lossy(parts[1]).to_string();
  let body = String::from_utf8_lossy(parts[2]).to_string();
  
  RequestMessage {
      id,
      return_address,
      resource_indicator,
      body,
  }
}

pub fn encode_confirmation(message: &ConfirmationMessage) -> Vec<u8> {
  let mut message_buffer = Vec::new();
  message_buffer.extend_from_slice(&message.id.to_be_bytes());
  message_buffer.extend_from_slice(&message.status_code.to_be_bytes());
  message_buffer.extend_from_slice(message.return_address.as_bytes());
  message_buffer.push(0);
  message_buffer
}

pub fn encode_response(message: &ResponseMessage) -> Vec<u8> {
  let mut message_buffer = Vec::new();
  message_buffer.extend_from_slice(&message.id.to_be_bytes());
  message_buffer.extend_from_slice(&message.status_code.to_be_bytes());
  message_buffer.extend_from_slice(message.return_address.as_bytes());
  message_buffer.push(0);
  message_buffer.extend_from_slice(message.body.as_bytes());
  message_buffer.push(0);
  message_buffer
}

