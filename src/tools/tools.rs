use std::fs;

pub fn read_input(path: String) -> String {
    let contents = fs::read_to_string(path)
        .expect("Failed to read file");
    contents
}

pub fn read_input_reference(path: &str) -> String {
    let contents = fs::read_to_string(path)
        .expect("Failed to read file");
    contents
}