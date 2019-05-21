use json::JsonValue;
use regex::Regex;

pub struct TSPaths<'a> {
    ts_paths: &'a JsonValue,
}

pub type TSPathsVec = Vec<(Regex, (String, String))>;

impl<'a> TSPaths<'a> {
    pub fn new(ts_paths: &JsonValue) -> TSPaths {
        TSPaths { ts_paths }
    }

    /// Creates a TS Paths vec
    pub fn as_vec(&self) -> TSPathsVec {
        let mut tspaths_vec: TSPathsVec = Vec::new();

        for tspath in self.ts_paths.entries() {
            let from = tspath.0.replace("*", "");
            let to = tspath.1[0].to_string().replace("*", "");;

            let regex_pattern = &format!("{}{}{}", "require\\(\'+([", from, "])(.)+\'\\)");

            dbg!(regex_pattern);

            let regx = Regex::new(regex_pattern).unwrap();

            tspaths_vec.push((regx, (from, to)));
        }

        tspaths_vec
    }
}
