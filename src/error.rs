use reqwest;
use serde_json;
use std;
use std::convert::From;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJSON(serde_json::Error),
    Http(reqwest::StatusCode),
    NoSession,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let msg = match self {
            Self::Reqwest(err) => err.to_string(),
            Self::SerdeJSON(err) => err.to_string(),
            Self::Http(status) => format!("Http error with status code: {}", status),
            Self::NoSession => String::from("Not logged into WebUntis"),
        };

        formatter.write_str(&msg)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Reqwest(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeJSON(err)
    }
}
