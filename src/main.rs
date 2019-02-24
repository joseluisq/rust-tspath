extern crate glob;
#[macro_use]
extern crate lazy_static;
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

fn apply_replacement(line: &str, replacement: &str) {
    info!("- APPLY REPLACED!");
}

fn process_line(line: &str) {
    // TODO: Load compilerOptions.paths (tsconfig.json)

    // TODO: Apply replacement and save file with new changes
    apply_replacement(&line, "~/b");
}

/// Reads a file by path
fn read_file(path: &PathBuf) {
    let display = path.display();

    let buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => BufReader::new(file),
    };

    lazy_static! {
        static ref RE: Regex = Regex::new(r"require\('./[a-zA-Z-_.]'\)").unwrap();
    };

    for (index, line) in buf_reader.lines().enumerate() {
        match line {
            Err(e) => warn!("{:?}", e),
            Ok(line) => {
                info!("- FILE: {:?}", display);
                info!("- LINE {}: {:?}", index, &line);

                if RE.is_match(&line) {
                    info!("- PATH MATCH!");
                    process_line(&line);
                }
            }
        }
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

fn main() {
    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    let source = get_argument(&args, "--source", "./");
    let config = get_argument(&args, "--config", "tsconfig.json");

    warn!("SOURCE: {}", &source);
    warn!("CONFIG: {}", &config);

    info!("--------");

    read_dir(&source);
}
