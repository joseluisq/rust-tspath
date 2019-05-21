use log::info;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

/// Writes a file
pub struct FileWriter<'a> {
    path: &'a PathBuf,
}

impl<'a> FileWriter<'a> {
    /// Creates an instance of FileReader
    pub fn new(path: &PathBuf) -> FileWriter {
        FileWriter { path }
    }

    /// Save a new file
    pub fn save(&self, new_data: &str) {
        let display = &self.path.display();

        let mut file = match File::create(&self.path) {
            Err(e) => panic!("couldn't create {}: {}", display, e),
            Ok(file) => file,
        };

        match file.write_all(&new_data.as_bytes()) {
            Err(e) => panic!("couldn't write to {}: {}", display, e),
            Ok(_) => info!("successfully wrote to {}", display),
        }
    }
}
