#[derive(Debug)]
pub enum Error {
    HTTPError(reqwest::Error),
    HTTPStatusError(reqwest::StatusCode),
    JSONError(serde_json::Error),
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
