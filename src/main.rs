extern crate glob;
#[macro_use]
extern crate lazy_static;
extern crate json;
extern crate log;
extern crate regex;
extern crate simple_logger;

use glob::glob;
use log::{info, warn};
use regex::Regex;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::PathBuf;

/// Gets one process argument by key and it also supports a default value
fn get_argument(args: &[String], key: &str, defaults: &str) -> String {
    let mut val = String::from("");
    let mut check = false;

    for arg in args {
        if check {
            val.push_str(&arg);
            break;
        }

        if !check && arg == key {
            check = true
        }
    }

    if val == "" {
        defaults.to_owned()
    } else {
        val
    }
}

fn process_line(line: &str) -> String {
    // TODO: Use real data from tsconfig.json
    let from = "~/b";
    let to = "./b";

    line.replace(&*from, &*to)
}

/// Reads a file line by line
fn read_file(path: &PathBuf) {
    let display = path.display();

    let buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => BufReader::new(file),
    };

    lazy_static! {
        // TODO:
        //  - use a global variable for Regex format value
        //  - fix regex format value
        static ref RE: Regex = Regex::new(r"require\('[~/b]'\)").unwrap();
    };

    let mut new_data: String = String::from("");
    let mut has_changes: bool = false;

    for line in buf_reader.lines() {
        match line {
            Err(e) => warn!("{:?}", e),
            Ok(line) => {
                // info!("- FILE: {:?}", display);
                // info!("- LINE {}: {:?}", index, &line);

                if RE.is_match(&line) {
                    info!("- FILE: {:?} - LINE REPLACED!", display);

                    let new_line = process_line(&line);

                    new_data.push_str(&new_line);
                    has_changes = true;
                } else {
                    new_data.push_str(&line);
                }

                new_data.push_str("\n");
            }
        }
    }

    if has_changes && !new_data.is_empty() {
        save_file(&path, new_data);
    }
}

// Save a new file
fn save_file(path: &PathBuf, new_data: String) {
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(new_data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

/// Scans a directory by glob pattern
fn read_dir(pattern: &str) {
    for entry in glob(pattern).expect("Failed to read glob pattern") {
        match entry {
            Err(e) => warn!("{:?}", e),
            Ok(path) => read_file(&path),
        }
    }
}

/// Reads a tsconfig.json file
fn read_tsconfig_file(path: &str) -> Result<json::JsonValue, json::Error> {
    let mut buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", path, e),
        Ok(file) => BufReader::new(file),
    };

    let mut contents = String::new();
    buf_reader
        .read_to_string(&mut contents)
        .expect("Unable to read the file");

    let data = match json::parse(&contents) {
        Err(e) => panic!("couldn't parse file {}: {}", path, e),
        Ok(contents) => contents,
    };

    Ok(data)
}

fn main() {
    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    let source = get_argument(&args, "--source", "./");
    let config = get_argument(&args, "--config", "tsconfig.json");

    info!("SOURCE: {}", &source);
    info!("CONFIG: {}", &config);
    info!("--------");

    let data = match read_tsconfig_file(&config) {
        Err(e) => panic!("couldn't parse json data: {}", e),
        Ok(v) => v,
    };

    // TODO: Checks required properties
    info!("JSON: {:?}", data["compilerOptions"]);

    read_dir(&source);
}
