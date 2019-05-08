extern crate glob;
extern crate json;
extern crate lazy_static;
extern crate log;
extern crate regex;
extern crate simple_logger;

use std::ffi::OsStr;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

use log::info;
use regex::Regex;

mod args;
mod scanfs;

use crate::args::Args;
use crate::scanfs::ScanDir;

/// Reads a file line by line
fn read_file(path: &PathBuf, _ts_path: &std::path::Path, ts_paths: &json::JsonValue) {
    let display = path.display();

    let buf_reader = match File::open(&path) {
        Err(e) => panic!("couldn't open {}: {}", display, e),
        Ok(file) => BufReader::new(file),
    };

    // lazy_static! {
    // TODO:
    //  - use a global variable for Regex format value
    //  - fix regex format value
    // };

    let mut tspaths_vec: Vec<(Regex, (String, String))> = Vec::new();

    // TODO: verify Regex pattern
    for tspath in ts_paths.entries() {
        let from = tspath.0.replace("*", "");
        let to = tspath.1.to_string().replace("*", "");

        let pattern = &format!(r"{}{}{}", "require('[", from, "]')");

        dbg!(pattern);

        let regx = Regex::new(pattern).unwrap();

        tspaths_vec.push((regx, (from, to)));
    }

    let mut new_data: String = String::from("");
    let mut has_changes: bool = false;

    for line in buf_reader.lines() {
        match line {
            Err(e) => panic!("{:?}", e),
            Ok(line) => {
                // info!("- FILE: {:?}", display);
                // info!("- LINE {}: {:?}", index, &line);

                // TODO: checks Regex intering `tspaths_vec`
                for p in &tspaths_vec {
                    let regx = &p.0;

                    if regx.is_match(&line) {
                        info!("- FILE: {:?} - LINE REPLACED!", display);
                        let jsonv = &p.1;
                        let from = &jsonv.0;
                        let to = &jsonv.1;

                        let new_line = line.replace(&*from, &*to);

                        new_data.push_str(&new_line);
                        has_changes = true;
                    } else {
                        new_data.push_str(&line);
                    }
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
        Err(e) => panic!("couldn't create {}: {}", display, e),
        Ok(file) => file,
    };

    match file.write_all(new_data.as_bytes()) {
        Err(e) => panic!("couldn't write to {}: {}", display, e),
        Ok(_) => info!("successfully wrote to {}", display),
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
        Err(e) => panic!("couldn't parse {}: {}", path, e),
        Ok(contents) => contents,
    };

    Ok(data)
}

fn main() {
    simple_logger::init().unwrap();

    let args = Args::new();
    let ts_source = args.get("--source", "./");
    let ts_config = args.get("--config", "tsconfig.json");

    info!("SOURCE: {}", &ts_source);
    info!("CONFIG: {}", &ts_config);
    info!("--------");

    let tsconfig = match read_tsconfig_file(&ts_config) {
        Err(e) => panic!("couldn't parse json data: {}", e),
        Ok(v) => v,
    };

    let ts_compiler_options = &tsconfig["compilerOptions"];

    if ts_compiler_options.is_empty() || !ts_compiler_options.is_object() {
        panic!("`compilerOptions` property is not a valid object or empty")
    }

    let ts_base_url = &ts_compiler_options["baseUrl"];

    if ts_base_url.is_empty() || !ts_base_url.is_string() {
        panic!("`baseUrl` property is not defined or empty")
    }

    let ts_paths = &ts_compiler_options["paths"];

    if ts_paths.is_empty() || !ts_paths.is_object() {
        panic!("`paths` property is not a valid object or empty")
    }

    let os_base_url_str = &ts_base_url.as_str();
    let os_base_url_path = OsStr::new(os_base_url_str.unwrap());
    let os_ts_base_path = Path::new(os_base_url_path);

    let scan_dir = ScanDir::new(&ts_source);
    scan_dir.scan(|path: &PathBuf| read_file(&path, &os_ts_base_path, &ts_paths));
}
