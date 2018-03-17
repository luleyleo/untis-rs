use std;
use std::fmt::{self, Display, Formatter};
use std::convert::From;
use reqwest;
use serde_json;

#[derive(Debug)]
pub enum Error {
    Reqwest(reqwest::Error),
    SerdeJSON(serde_json::Error),
    Http(reqwest::StatusCode),
    NoSession,
}

impl Display for Error {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        let msg = match *self {
            Error::Http(ref status) => format!("Http error with status code: {}", status),
            _ => std::error::Error::description(self).to_owned(),
        };

        formatter.write_str(&msg)
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Reqwest(ref err) => err.description(),
            Error::SerdeJSON(ref err) => err.description(),
            Error::Http(_) => "The Http request didn't succeed.",
            Error::NoSession => "This method can't be called without a session.",
        }
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
