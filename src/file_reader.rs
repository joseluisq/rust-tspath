use json::JsonValue;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

use line_replacer::LineReplacer;
use tspaths::TSPaths;

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
    pub fn read<F>(&self, ts_paths: &JsonValue, mut fun: F)
    where
        F: FnMut(&PathBuf, &String) -> (),
    {
        let display = &self.path.display();

        let buf_reader = match File::open(&self.path) {
            Err(e) => panic!("couldn't open {}: {}", display, e),
            Ok(file) => BufReader::new(file),
        };

        let tspaths_vec = TSPaths::new(ts_paths).as_vec();

        let mut new_lines = String::new();

        for line in buf_reader.lines() {
            match line {
                Err(e) => panic!("{:?}", e),
                Ok(line) => {
                    LineReplacer::new(line, &tspaths_vec).replace(|new_line: String| {
                        new_lines.push_str(&new_line);
                    });
                }
            }
        }

        fun(&self.path, &new_lines);
    }
}
