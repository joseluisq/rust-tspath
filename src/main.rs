extern crate glob;
extern crate json;
extern crate lazy_static;
extern crate log;
extern crate regex;
extern crate simple_logger;

use log::info;
//use std::ffi::OsStr;
use std::path::PathBuf;

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
use tsconfig::TSConfig;

fn main() {
    simple_logger::init().unwrap();

    let args = CLIArgs::new();
    let str_ts_source = args.get("--source", "./");
    let str_ts_config = args.get("--config", "tsconfig.json");

    info!("SOURCE: {}", &str_ts_source);
    info!("CONFIG: {}", &str_ts_config);
    info!("--------");

    replace_ts_file_paths(&str_ts_source, &str_ts_config);
}

fn replace_ts_file_paths(str_ts_source_path: &str, str_ts_config_path: &str) {
    // process a tsconfig file
    let tsconfig = TSConfig::new(&str_ts_config_path);

    let json_tsconfig = match tsconfig.read() {
        Err(e) => panic!("couldn't parse json data: {}", e),
        Ok(v) => v,
    };

    // tsconfig: compiler options
    let ts_compiler_options = &json_tsconfig["compilerOptions"];

    if ts_compiler_options.is_empty() || !ts_compiler_options.is_object() {
        panic!("`compilerOptions` property is not a valid object or empty")
    }

    // tsconfig: base url
    let ts_base_url = &ts_compiler_options["baseUrl"];

    if ts_base_url.is_empty() || !ts_base_url.is_string() {
        panic!("`baseUrl` property is not defined or empty")
    }

    // tsconfig: ts paths
    let ts_paths = &ts_compiler_options["paths"];

    if ts_paths.is_empty() || !ts_paths.is_object() {
        panic!("`paths` property is not a valid object or empty")
    }

    info!("TS PATH: {}", &ts_paths);

    //    let os_base_url_str = &ts_base_url.as_str();
    //    let os_base_url_path = OsStr::new(os_base_url_str.unwrap());

    // It only called when there is new data available
    let save_file = |path: &PathBuf, new_data: &String| {
        FileWriter::new(&path).save(&new_data);
    };

    // Reads file by file
    let read_file = |path: &PathBuf| {
        FileReader::new(&path).read(&ts_paths, save_file);
    };

    // Reads TS source directory
    DirReader::new(&str_ts_source_path).read(read_file);
}
