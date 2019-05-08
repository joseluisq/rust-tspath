use log::info;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

/// Reads a file
pub struct FileReader<'a> {
    path: &'a PathBuf,
}

impl<'a> FileReader<'a> {
    /// Creates an instance of FileReader
    pub fn new(path: &PathBuf) -> FileReader {
        FileReader { path }
    }

    /// Reads a file line by line and return the data if it matches with the Regex
    pub fn read<F>(&self, _ts_path: &std::path::Path, ts_paths: &json::JsonValue, fun: F)
    where
        F: Fn(&PathBuf, String) -> (),
    {
        let display = &self.path.display();

        let buf_reader = match File::open(&self.path) {
            Err(e) => panic!("couldn't open {}: {}", display, e),
            Ok(file) => BufReader::new(file),
        };

        // lazy_static! {
        // TODO:
        //  - use a global variable for Regex format value
        //  - fix regex format value
        // };

        // TODO: Split Regex functionality
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

                    // TODO: checks Regex `tspaths_vec`
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
            fun(&self.path, new_data);
        }
    }
}
