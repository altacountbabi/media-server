#![allow(dead_code)] // TODO: Use this

use bcrypt::DEFAULT_COST;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Password {
    pub hashed: String,
}

impl Password {
    pub fn new(password: impl Into<String>) -> Result<Self, bcrypt::BcryptError> {
        let password: String = password.into();
        let hashed = bcrypt::hash(&password, DEFAULT_COST)?;

        Ok(Self { hashed })
    }

    pub fn verify(&self, password: impl Into<String>) -> Result<bool, bcrypt::BcryptError> {
        bcrypt::verify(password.into(), &self.hashed)
    }
}
