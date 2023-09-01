//! Library to access [Untis](https://www.untis.at)
//!
//! The core of this crate is the `Untis` struct.

#![feature(let_chains)]

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod client;
mod datetime;
mod error;
mod params;
mod resources;
mod school_search;

pub mod jsonrpc;
pub use client::Client;
pub use datetime::*;
pub use error::Error;
pub use resources::*;
pub use school_search::*;
