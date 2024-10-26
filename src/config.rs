use log::info;
use serde::Deserialize;
use std::process::Command;

use crate::library::Library;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_keys: ApiKeys,
    pub libraries: Vec<Library>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeys {
    pub omdb: String,
}

pub fn eval_config() -> Result<Config, Box<dyn std::error::Error>> {
    info!("Evaluating configuration file");

    let init = include_str!("init.nix").replace("\"$CONFIG\"", "./config");
    let output = Command::new("nix-instantiate")
        .args(["--eval", "--strict", "--json", "--expr", &init])
        .output()?;

    if !output.status.success() {
        panic!("Failed to evaluate config file:\n{}", String::from_utf8_lossy(&output.stderr));
    }

    let config_json = String::from_utf8(output.stdout)?;
    serde_json::from_str(&config_json).map_err(Into::into)
}
