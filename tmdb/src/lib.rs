pub struct TMDb {
    api_key: String,
    language: String,
}

impl TMDb {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            language: "en".into(),
        }
    }

    pub fn with_language(api_key: impl Into<String>, language: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            language: language.into(),
        }
    }
}
