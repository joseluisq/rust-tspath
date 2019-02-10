extern crate log;
extern crate simple_logger;

use log::warn;
use std::env;

fn get_argument(key: &str, args: &[String], defaults: &str) -> String {
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

fn main() {
    simple_logger::init().unwrap();

    let args: Vec<String> = env::args().collect();

    let source = get_argument("--source", &args, "./");
    let config = get_argument("--config", &args, "tsconfig.json");

    warn!("SOURCE: {}", source);
    warn!("CONFIG: {}", config);
}
