use json::JsonValue;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// TODO: Implement an struct instead of functions

/// Reads a tsconfig.json file
pub fn read_tsconfig_file(path: &str) -> Result<JsonValue, json::Error> {
    let mut buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", path, e),
        Ok(file) => BufReader::new(file),
    };

    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read the file");

    let data = match json::parse(&contents) {
        Err(e) => panic!("couldn't parse {}: {}", path, e),
        Ok(contents) => contents,
    };

    Ok(data)
}
