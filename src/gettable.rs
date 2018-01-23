use std;

pub trait Gettable: std::marker::Sized {
    fn _get(&self, key: &String) -> Option<&Self>;
}
