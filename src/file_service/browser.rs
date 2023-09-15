use walkdir::WalkDir;
use crate::message_handler::interpreter::ResponseMessage;

pub fn list_files_in_directory(request_body: &str, response: &mut ResponseMessage) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let path = request_body.to_string();
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry?; // Propagate error upwards using ? operator instead of unwrap
        files.push(entry.path().display().to_string());
    }
    response.status_code = 200;
    response.body = files.join("\n");
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files_in_directory() {
        let files = list_files_in_directory(".");
        assert!(!files.is_empty());
    }
}
