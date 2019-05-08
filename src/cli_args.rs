use std::env;

/// CLI Arguments Reader
pub struct CLIArgs {
    args: Vec<String>,
}

impl CLIArgs {
    /// Creates an instance of Args
    pub fn new() -> CLIArgs {
        let args_vec: Vec<String> = env::args().collect();
        CLIArgs { args: args_vec }
    }

    /// Gets one CLI argument by key accepting an optional default value
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
