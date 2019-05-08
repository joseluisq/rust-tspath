use std::env;

pub struct Args {
    args: Vec<String>,
}

impl Args {
    pub fn new() -> Args {
        let args_vec: Vec<String> = env::args().collect();

        Args { args: args_vec }
    }

    /// Gets one process argument by key and it also supports a default value
    pub fn get(&self, key: &str, defaults: &str) -> String {
        let mut val = String::from("");
        let mut check = false;

        for arg in &self.args {
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
}
