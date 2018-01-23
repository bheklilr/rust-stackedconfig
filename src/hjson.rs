extern crate serde_hjson;
use serde_hjson::de::*;

use gettable::{Gettable};


impl Gettable for serde_hjson::Value {
    fn _get(&self, key: &String) -> Option<&Self> {
        self.find(key)
    }
}


mod tests {
    use super::super::configstack::*;
    use serde_hjson::{Result,Value,from_str};

    #[test]
    fn test_lookup() {
        fn work() -> Result<ConfigStack<Value>> {
            let stack: ConfigStack<Value> = ConfigStack::new();

            let s1 = r#"
                a:
                    b:
                        c:
                            d:
                                - 1
                                - 2
                                - 3
                    e: 1
            "#;
            let v1: Value = from_str(s1)?;

            let s2 = r#"
                a:
                    b:
                        c: 2
            "#;
            let v2: Value = from_str(s2)?;
            Ok(stack.push(v1).push(v2))
        }

        match work() {
            Ok(stack) => {
                let data = from_str("2").unwrap();
                let expected: Lookup<&Value> = Lookup::Found(&data);
                let actual = stack.get("a/b/c");
                assert_eq!(actual, expected);
            },
            _ => assert!(false),
        }
    }
}
