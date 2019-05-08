use glob::glob;
use std::path::PathBuf;

pub struct ScanDir<'a> {
    pattern: &'a str,
}

impl<'a> ScanDir<'a> {
    pub fn new(pattern: &str) -> ScanDir {
        ScanDir { pattern }
    }

    /// Scans a directory by glob pattern
    pub fn scan<F>(&self, fun: F)
    where
        F: Fn(&PathBuf) -> (),
    {
        for entry in glob(&self.pattern).expect("Failed to read glob pattern") {
            match entry {
                Err(e) => panic!("{:?}", e),
                Ok(path) => fun(&path),
            }
        }
    }
}
