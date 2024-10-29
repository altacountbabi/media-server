use crate::library::Library;
use log::{debug, error};
use serde::Deserialize;
use std::{
    path::PathBuf,
    process::{self, Command},
};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub data_dir: PathBuf,
    pub libraries: Vec<Library>,
}

pub fn eval_config(config_path: PathBuf) -> Result<Config, Box<dyn std::error::Error>> {
    if let Err(_) = dotenv::from_path(config_path.join(".env")) {
        error!("Failed to load .env file");
        process::exit(1);
    }

    debug!("Evaluating configuration file");

    let init = include_str!("init.nix").replace("\"$CONFIG\"", &config_path.canonicalize()?.to_string_lossy().to_string());
    let output = Command::new("nix-instantiate")
        .args(["--eval", "--strict", "--json", "--expr", &init])
        .output()?;

    if !output.status.success() {
        panic!("Failed to evaluate config file:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    let config_json = String::from_utf8(output.stdout)?;
    serde_json::from_str(&config_json).map_err(Into::into)
}
