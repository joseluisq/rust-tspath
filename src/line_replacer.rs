use tspaths::TSPathsVec;

pub struct LineReplacer<'a> {
    line: String,
    ts_paths_vec: &'a TSPathsVec,
}

impl<'a> LineReplacer<'a> {
    pub fn new(line: String, ts_paths_vec: &TSPathsVec) -> LineReplacer {
        LineReplacer { line, ts_paths_vec }
    }

    pub fn replace<F>(&self, mut fun: F)
    where
        F: FnMut(String) -> (),
    {
        let mut new_data: String = String::new();
        let mut has_changes: bool = false;

        for p in self.ts_paths_vec {
            let regx = &p.0;

            if regx.is_match(&self.line) {
                let jsonv = &p.1;
                let from = &jsonv.0;
                let to = &jsonv.1;

                let new_line = &self.line.replace(&*from, &*to);

                new_data.push_str(&new_line);
                has_changes = true;
            } else {
                new_data.push_str(&self.line);
            }
        }

        new_data.push_str("\n");

        if has_changes && !new_data.is_empty() {
            fun(new_data);
        }
    }
}
