use crate::library::Library;
use log::{debug, error};
use serde::Deserialize;
use std::{fs, path::PathBuf, process};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Config {
    pub data_dir: PathBuf,
    #[serde(rename = "library")]
    pub libraries: Vec<Library>,
}

pub fn read_config(config_path: PathBuf) -> Config {
    if let Err(err) = dotenv::from_path(config_path.join(".env")) {
        error!("Failed to load .env file: {err}");
        process::exit(1);
    }

    debug!("Parsing config file");
    let raw = match fs::read_to_string(config_path.join("config.toml")) {
        Ok(raw) => raw,
        Err(err) => {
            error!("Failed to read config file: {err}");
            process::exit(1);
        }
    };

    match toml::from_str(&raw) {
        Ok(config) => config,
        Err(err) => {
            error!("Failed to parse config file: {err}");
            process::exit(1);
        }
    }
}
