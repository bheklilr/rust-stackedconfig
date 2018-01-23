#[cfg(feature = "json")]
#[macro_use]
extern crate serde_json;
#[cfg(feature = "yaml")]
extern crate serde_yaml;
#[cfg(feature = "hjson")]
extern crate serde_hjson;

mod gettable;
mod configstack;
use gettable::{Gettable};
use configstack::{ConfigStack, Lookup};

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "BSON")]
mod bson;
#[cfg(feature = "hjson")]
mod hjson;

