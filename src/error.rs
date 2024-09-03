use std::convert::From;
use std::fmt::{self, Display, Formatter};

use crate::jsonrpc;

/// Represents all errors that can occur during an Untis API request.
#[derive(Debug)]
pub enum Error {
    /// Error during the request itself.
    Reqwest(reqwest::Error),

    /// Error while serializing/parsing data.
    Serde(serde_json::Error),

    /// Error with the response HTTP status code.
    Http(reqwest::StatusCode),

    /// The RPC response contained an error.
    Rpc(jsonrpc::Error),

    /// No results were found.
    NotFound,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let msg = match self {
            Self::Reqwest(err) => format!("Reqwest error: {}", err),
            Self::Serde(err) => format!("Serde Error: {}", err),
            Self::Http(status) => format!("HTTP Error: {}", status),
            Self::Rpc(error) => format!("RPC Error: {} {}", error.code, error.message),
            Self::NotFound => String::from("Resource not found"),
        };

        formatter.write_str(&msg)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::Serde(err)
    }
}
