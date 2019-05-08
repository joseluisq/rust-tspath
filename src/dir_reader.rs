use glob::glob;
use std::path::PathBuf;

/// Directory reader with glob pattern support
pub struct DirReader<'a> {
    /// A glob string pattern
    pattern: &'a str,
}

impl<'a> DirReader<'a> {
    /// Creates an instance of DirReader using a given glob pattern
    pub fn new(pattern: &str) -> DirReader {
        DirReader { pattern }
    }

    /// Scans one directory using the a pre-defined glob pattern
    pub fn read<F>(&self, fun: F)
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
