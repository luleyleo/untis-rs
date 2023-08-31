use reqwest;
use serde_json;
use std;
use std::convert::From;
use std::fmt::{self, Display, Formatter};

use crate::jsonrpc;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Http(reqwest::StatusCode),
    Rpc(jsonrpc::Error),
    NotFound,
    NoSession,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let msg = match self {
            Self::Reqwest(err) => format!("Reqwest error: {}", err.to_string()),
            Self::Serde(err) => format!("Serde Error: {}", err.to_string()),
            Self::Http(status) => format!("HTTP Error: {}", status),
            Self::Rpc(error) => format!("RPC Error: {} {}", error.code, error.message),
            Self::NotFound => String::from("Resource not found"),
            Self::NoSession => String::from("Client not authenticated"),
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
