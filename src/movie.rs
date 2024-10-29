use crate::{
    cache::Cache,
    utils::{filename, re},
};
use log::{debug, error};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tmdb::{models, TMDb};

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub path: PathBuf,
    pub metadata: tmdb::models::Movie,
}

pub async fn fetch_info(path: PathBuf, cache: &Cache, skip_cache: bool, tmdb: &TMDb) -> Result<(bool, Movie), tmdb::Error> {
    let name = &filename(&path);

    if skip_cache {
        debug!("Skipping cache");
    }

    if !skip_cache && let Ok(Some(cached_movie)) = cache.get_movie(name).await {
        debug!("Found '{}' in cache", name);
        return Ok((true, cached_movie));
    }

    // Parse movie name
    let (title, year) = re(r"^(.*) \((\d{4})\)$")
        .captures(name)
        .map(|caps| {
            (
                caps[1].trim().to_string(), // Capture the name
                caps[2].to_string(),        // Capture the year
            )
        })
        .expect("Failed to parse movie name");

    // Fetch movie metadata
    let mut tmdb_search: models::MovieSearchResults = match tmdb.search(&title).year(year).execute().await {
        Ok(search) => search,
        Err(e) => {
            error!("Failed to fetch metadata from for movie '{title}' from TMDb: {:#?}", e);
            return Err(e);
        }
    };
    let tmdb_metadata = tmdb_search.results.remove(0);

    Ok((
        false,
        Movie {
            path,
            metadata: tmdb_metadata,
        },
    ))
}
