//! Library to access [Untis](https://www.untis.at)
//!
//! The core of this crate is the `Untis` struct.

extern crate chrono;
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

mod date;
mod error;
mod jsonrpc;
mod request;
mod response;
mod time;
mod untis;

pub use date::UntisDate;
pub use error::Error;
pub use request::*;
pub use response::*;
pub use time::UntisTime;
pub use untis::Units;
