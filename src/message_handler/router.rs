// message_handler/router.rs
use crate::tcp_interface::client::send_byte_message;
use crate::message_handler::interpreter::{
    decode_request,
    encode_confirmation,
    encode_response,
    RequestMessage,
    ConfirmationMessage,
    ResponseMessage,
};
use crate::file_service::launcher::open_file;
use crate::file_service::browser::list_files_in_directory;

pub fn handle_message(message_buffer: &[u8]) {
    let message = decode_request(message_buffer);
    if route_message(&message) {
        let confirmation_message = ConfirmationMessage {
            id: message.id,
            return_address: message.return_address.clone(),
            status_code: 200,
        };
        let confirmation_message_buffer = encode_confirmation(&confirmation_message);
        if let Err(e) = send_byte_message(&confirmation_message_buffer, &message.return_address) {
            // Handle the error e, perhaps logging it or taking corrective action.
        }
    } else {
        let confirmation_message = ConfirmationMessage {
            id: message.id,
            return_address: message.return_address.clone(),
            status_code: 400,
        };
        let confirmation_message_buffer = encode_confirmation(&confirmation_message);
        if let Err(e) = send_byte_message(&confirmation_message_buffer, &message.return_address) {
            // Handle the error e, perhaps logging it or taking corrective action.
        }
    }
}

fn route_message(message: &RequestMessage) -> bool {
    let mut response_message = ResponseMessage {
        id: message.id,
        return_address: message.return_address.clone(),
        status_code: 0,
        body: "".to_string(),
    };

    if message.resource_indicator == "list_files_in_directory" {
        match list_files_in_directory(&message.body, &mut response_message) {
            Ok(files) => {
                //convert Vec<String> to String
                response_message.body = files.join("\n");
            },
            Err(e) => {
                println!("Error: {}", e);
            },
        }
    } else if message.resource_indicator == "open_file" {
        if let Err(e) = open_file(&message.body, &mut response_message) {
            response_message.status_code = 500;
            response_message.body = format!("Error: {}", e);
        }
    } else {
        response_message.status_code = 404;
        response_message.body = "Resource not found".to_string();
    }

    let response_message_buffer = encode_response(&response_message);
    if let Err(e) = send_byte_message(&response_message_buffer, &message.return_address) {
        // Handle the error e, perhaps logging it or taking corrective action.
    }
    true
}
