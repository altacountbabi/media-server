use crate::{
    cache::Cache,
    utils::{filename, re},
};
use log::trace;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tmdb::TMDb;

#[derive(Debug, Deserialize, Serialize)]
pub struct Movie {
    pub path: PathBuf,
    // pub metadata: MovieMetadata,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct MovieMetadata {
    pub tmdb_id: u64,
    pub title: String,
    pub release_date: String,
}

pub async fn fetch_info(path: PathBuf, cache: &Cache, skip_cache: bool, tmdb_client: &TMDb) -> Movie {
    let name = &filename(&path);

    if !skip_cache {
        if let Ok(Some(cached_movie)) = cache.get_movie(name).await {
            trace!("Found '{}' in cache", name);
            return cached_movie;
        }
    }

    // Parse movie name
    let (title, year) = re(r"^(.*) \((\d{4})\)$")
        .captures(name)
        .map(|caps| {
            (
                caps[1].trim().to_string(),                                        // Capture the name
                caps[2].to_string().parse::<u64>().expect("Failed to parse year"), // Capture the year
            )
        })
        .expect("Failed to parse movie name");

    // Fetch movie metadata
    // let tmdb_search = tmdb
    //     .search()
    //     .title(&title)
    //     .year(year)
    //     .execute()
    //     .expect("Failed to fetch metadata from TMDb");

    // let tmdb_metadata = tmdb_search.results.first().expect("No movie found").to_owned();

    // let metadata = MovieMetadata {
    //     tmdb_id: tmdb_metadata.id,
    //     title: tmdb_metadata.title.to_string(),
    //     release_date: tmdb_metadata.release_date.to_string(),
    // };

    Movie { path }
}
