#[cfg(feature = "json")]
#[macro_use]
extern crate serde_json;

mod gettable;
mod configstack;

#[cfg(feature = "json")]
mod json;
#[cfg(feature = "yaml")]
mod yaml;
#[cfg(feature = "BSON")]
mod bson;
#[cfg(feature = "hjson")]
mod hjson;

pub use gettable::{Gettable};
pub use configstack::{ConfigStack, Lookup};
