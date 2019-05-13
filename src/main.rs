extern crate glob;
extern crate json;
extern crate lazy_static;
extern crate log;
extern crate regex;
extern crate simple_logger;

use log::info;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

mod cli_args;
mod dir_reader;
mod file_reader;
mod file_writer;
mod line_replacer;
mod tsconfig;
mod tspaths;

use cli_args::CLIArgs;
use dir_reader::DirReader;
use file_reader::FileReader;
use file_writer::FileWriter;

fn main() {
    simple_logger::init().unwrap();

    let args = CLIArgs::new();
    let ts_source = args.get("--source", "./");
    let ts_config = args.get("--config", "tsconfig.json");

    info!("SOURCE: {}", &ts_source);
    info!("CONFIG: {}", &ts_config);
    info!("--------");

    let tsconfig = match tsconfig::read_tsconfig_file(&ts_config) {
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

    // It only called when there is new data available
    let save_file = |path: &PathBuf, new_data: String| FileWriter::new(&path).save(new_data);

    // Reads file by file
    let read_file = |path: &PathBuf| {
        FileReader::new(&path).read(&os_ts_base_path, &ts_paths, save_file);
    };

    // Reads TS source directory
    let dir_reader = DirReader::new(&ts_source);
    dir_reader.read(read_file);
}
