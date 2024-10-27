use crate::{
    cache::Cache,
    utils::{filename, re},
};
use log::trace;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub path: PathBuf,
    pub metadata: omdb::Movie,
}

pub async fn fetch_info(path: PathBuf, cache: &Cache, skip_cache: bool, omdb_api_key: &str) -> Movie {
    let name = &filename(&path);

    if !skip_cache {
        if let Ok(Some(cached_movie)) = cache.get_movie(name).await {
            trace!("Found '{}' in cache", name);
            return cached_movie;
        }
    }

    // Parse movie name
    let (title, release_year) = re(r"^(.*) \((\d{4})\)$")
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
