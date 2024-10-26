use regex::Regex;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub path: PathBuf,
    pub metadata: omdb::Movie,
}

pub async fn fetch_info(path: PathBuf, omdb_api_key: &str) -> Movie {
    let name = path
        .file_name()
        .expect("Failed to get file name")
        .to_str()
        .expect("Failed to convert file name to string");

    // Parse movie name
    let re = Regex::new(r"^(.*) \((\d{4})\)$").unwrap();
    let (title, release_year): (String, String) = re
        .captures(name)
        .map(|caps| {
            (
                caps[1].trim().to_string(), // Capture the name
                caps[2].to_string(),        // Capture the year
            )
        })
        .expect("Failed to parse movie name");

    // Fetch movie metadata
    let metadata = omdb::title(&title)
        .apikey(omdb_api_key)
        .year(&release_year)
        .get()
        .await
        .expect("Failed to fetch movie data from OMDb");

    Movie { path, metadata }
}
