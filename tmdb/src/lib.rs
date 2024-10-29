mod apis;
mod client;
pub mod types;
pub use types::error::Error;
pub use types::models;

#[derive(Clone)]
pub struct TMDb {
    pub(crate) api_key: String,
    pub(crate) language: String,

    pub(crate) reqwest: reqwest::Client,
}

impl TMDb {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self::with_language(api_key, "en-US")
    }

    pub fn with_language(api_key: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            language: language.into(),

            reqwest: reqwest::Client::new(),
        }
    }
}
