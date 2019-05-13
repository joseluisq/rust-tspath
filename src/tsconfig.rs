use json::JsonValue;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// TODO: Implement an struct instead of functions

pub struct TSConfig<'a> {
    path: &'a str,
}

impl<'a> TSConfig<'a> {
    pub fn new(path: &str) -> TSConfig {
        TSConfig { path }
    }

    /// Reads a tsconfig.json file
    pub fn read(&self) -> Result<JsonValue, json::Error> {
        let mut buf_reader = match File::open(self.path) {
            Err(e) => panic!("couldn't open {}: {}", self.path, e),
            Ok(file) => BufReader::new(file),
        };

        let mut contents = String::new();

        buf_reader
            .read_to_string(&mut contents)
            .expect("Unable to read the file");

        let data = match json::parse(&contents) {
            Err(e) => panic!("couldn't parse {}: {}", self.path, e),
            Ok(contents) => contents,
        };

        Ok(data)
    }
}
