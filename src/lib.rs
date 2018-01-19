#[macro_use]
extern crate serde_json;

use serde_json::{Value};

/// Combines multiple `serde_json::Value` objects together so they can be
/// queried as a single, nested object.
#[derive(Debug)]
pub struct ConfigStack {
    path_sep: char,
    configs: Vec<Value>,
}

/// Return value from looking up a path from a ConfigStack
///
///     conf.get("/foo/bar")      // Lookup::Found(1)
///     conf.get("/foo/bar/baz")  // Lookup::Missing
#[derive(Debug, PartialEq)]
pub enum Lookup {
    /// Indicates that the path did not resolve to a value
    Missing,
    /// Contains the `serde_json::Value` found from a lookup
    Found(Value),
}

impl ConfigStack {
    pub fn new() -> ConfigStack {
        ConfigStack { path_sep: '/', configs: vec![], }
    }

    pub fn with_path_sep(self, sep: char) -> ConfigStack {
        ConfigStack {
            path_sep: sep,
            configs: self.configs,
        }
    }

    pub fn push(mut self, config: Value) -> ConfigStack {
        self.configs.push(config);
        self
    }

    pub fn pop(mut self) -> Option<Value> {
        self.configs.pop()
    }

    fn path_to_parts(&self, path: &str) -> Vec<String> {
        let parts = path.trim_matches(self.path_sep).split(self.path_sep);
        parts.map(|s| s.to_string()).collect()
    }

    pub fn get(&self, path: &str) -> Lookup {
        self.get_parts(self.path_to_parts(path))
    }

    pub fn get_parts(&self, path_parts: Vec<String>) -> Lookup {
        'outer: for i in 0..self.configs.len() {
            let idx = self.configs.len() - i - 1;
            let mut node = &self.configs[idx];

            for part in path_parts.iter() {
                match node.get(part) {
                    Some(subobj) => node = subobj,
                    None => {
                        if idx == 0 {
                            return Lookup::Missing;
                        }
                        continue 'outer;
                    },
                }
            }
            return Lookup::Found(node.to_owned());
        }
        return Lookup::Missing;
    }
}


mod tests {
    use super::*;

    #[test]
    fn test_lookup() {
        fn work() -> serde_json::Result<ConfigStack> {
            let stack = ConfigStack::new();

            let s1 = r#"{
                "a": {
                    "b": {
                        "c": 1,
                        "d": [1, 2, 3]
                    },
                    "e": 1
                }
            }"#;
            let v1: Value = serde_json::from_str(s1)?;

            let s2 = r#"{
                "a": {
                    "b": {
                        "c": 2
                    }
                }
            }"#;
            let v2: Value = serde_json::from_str(s2)?;
            Ok(stack.push(v1).push(v2))
        }

        match work() {
            Ok(stack) => {
                let expected: Value = json!(2);
                assert_eq!(stack.get("a/b/c"), Lookup::Found(expected));
            },
            _ => assert!(false),
        }
    }
}
