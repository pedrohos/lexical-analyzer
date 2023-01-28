use std::fs;
use std::fs::File;
use std::io::Read;

pub fn read_from_file_path(file_path: &String) -> String {
    let mut file = File::open(file_path).unwrap_or_else(|e| {
        panic!("File non existent.");
    });
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    return String::from_utf8(contents).unwrap();
}