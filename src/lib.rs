//! Library to access [Untis](https://www.untis.at)
//!
//! The core of this crate is the `Untis` struct.

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod client;
mod datetime;
mod error;
pub mod jsonrpc;
pub mod params;
mod resources;
mod school_search;

pub use client::Client;
pub use datetime::*;
pub use error::Error;
pub use resources::*;
pub use school_search::*;
