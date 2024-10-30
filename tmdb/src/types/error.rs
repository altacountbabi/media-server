use std::io;

#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    HTTPStatusError(reqwest::StatusCode),
    JSONError(serde_json::Error),
    IOError(io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Self::HTTPError(err)
    }
}

impl From<reqwest::StatusCode> for Error {
    fn from(err: reqwest::StatusCode) -> Self {
        Self::HTTPStatusError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JSONError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self::IOError(err)
    }
}

impl From<io::ErrorKind> for Error {
    fn from(err: io::ErrorKind) -> Self {
        Self::IOError(err.into())
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::HTTPError(err) => err.fmt(f),
            Error::HTTPStatusError(err) => err.fmt(f),
            Error::JSONError(err) => err.fmt(f),
            Error::IOError(err) => err.fmt(f),
        }
    }
}
