extern crate serde;

use gettable::{Gettable};

/// Combines multiple `serde_json::Value` objects together so they can be
/// queried as a single, nested object.
///
/// Supports using a custom separator if desired for path style queries.
/// Defaults to a forward slash `/`.
#[derive(Debug)]
pub struct ConfigStack<Value> {
    path_sep: char,
    configs: Vec<Value>,
}

/// Return value from looking up a path from a [`ConfigStack`]
///
///     conf.get("/foo/bar")      // Lookup::Found(1)
///     conf.get("/foo/bar/baz")  // Lookup::Missing
///
/// [`ConfigStack`]: ./struct.ConfigStack.html
#[derive(Debug, PartialEq)]
pub enum Lookup<Value> {
    /// Indicates that the path did not resolve to a value
    Missing,
    /// Contains the `serde_json::Value` found from a lookup
    Found(Value),
}


impl<Value> ConfigStack<Value>
    where Value: Gettable
    {
    /// Create a new `ConfigStack`
    pub fn new() -> ConfigStack<Value> {
        ConfigStack { path_sep: '/', configs: vec![], }
    }

    /// Returns a new `ConfigStack` with the given separator
    ///
    /// ```rust
    /// let conf = ConfigStack::new().with_path_sep('.').push(v1).push(v2);
    /// let val = conf.get("foo.bar.baz");
    /// ```
    pub fn with_path_sep(self, sep: char) -> ConfigStack<Value> {
        ConfigStack {
            path_sep: sep,
            configs: self.configs,
        }
    }

    /// Adds a new configuration value to the stack.  The added value becomes
    /// the highest priority value on the stack.
    pub fn push(mut self, config: Value) -> ConfigStack<Value> {
        self.configs.push(config);
        self
    }

    /// Removes the top level configuration from the stack and returns it. The
    /// opposite of `push`.  If no configurations are on the stack then `None`
    /// is returned.
    pub fn pop(mut self) -> Option<Value> {
        self.configs.pop()
    }

    /// Converts a path to its constituent parts using the current path
    /// separator. Trims any leading or trailing separators
    fn path_to_parts(&self, path: &str) -> Vec<String> {
        let parts = path.trim_matches(self.path_sep).split(self.path_sep);
        parts.map(|s| s.to_string()).collect()
    }

    /// Looks up a value at the given path
    ///
    /// ```rust
    /// conf.get("foo/bar/baz") -> Lookup::Found(Value::Bool(true))
    /// conf.get("foo/bar/qux") -> Lookup::Missing
    /// ```
    pub fn get(&self, path: &str) -> Lookup<&Value> {
        self.get_parts(self.path_to_parts(path))
    }

    /// Looks up a value at the given path where the path is a `Vec<String>`. No
    /// parsing is performed on the path parts, so this method will not split on
    /// the path separator.
    pub fn get_parts(&self, path_parts: Vec<String>) -> Lookup<&Value> {
        'outer: for i in 0..self.configs.len() {
            let idx = self.configs.len() - i - 1;
            let mut node: &Value = &self.configs[idx];

            for part in path_parts.iter() {
                match node._get(part) {
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
