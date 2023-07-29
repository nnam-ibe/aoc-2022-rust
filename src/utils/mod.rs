pub fn file_to_string(location: &str) -> String {
    let file = std::fs::read_to_string(location).expect("File should exist");
    return file;
}
