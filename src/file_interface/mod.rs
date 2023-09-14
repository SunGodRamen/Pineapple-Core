use std::process::Command;
use walkdir::WalkDir;

pub fn list_files_in_directory(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in WalkDir::new(path) {
        let entry = entry.unwrap(); // Note: in a real application, handle this error gracefully rather than unwrapping.
        files.push(entry.path().display().to_string());
    }
    files
}


pub fn open_file(path: &str, app: Option<&str>) {
    if let Some(app) = app {
        Command::new(app)
            .arg(path)
            .spawn()
            .expect("Failed to open file with specific app");
    } else {
        Command::new("start")
            .arg(path)
            .spawn()
            .expect("Failed to open file");
    }
}
