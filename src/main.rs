mod file_browser;

fn list_files_in_directory(path: &str) -> Vec<String> {
    file_browser::list_files_in_directory(path)
}