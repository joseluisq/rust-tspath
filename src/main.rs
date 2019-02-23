extern crate glob;
extern crate log;
extern crate simple_logger;

use glob::glob;
use log::{info, warn};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;

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

fn read_file(path: &PathBuf) {
    let display = path.display();

    let buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => BufReader::new(file),
    };

    for (index, line) in buf_reader.lines().enumerate() {
        info!("- FILE: {:?}", display);
        info!("- LINE {}: {:?}", index, &line);

        // TODO: Match path comparin first and replacing
        match line {
            Err(e) => warn!("{:?}", e),
            Ok(line) => info!("- LINE {} (UP): {:?}", index, &line),
        }
    }
}

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
