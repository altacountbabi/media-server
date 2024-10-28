use crate::{
    cache::Cache,
    movie::{self, Movie},
};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tmdb::TMDb;
use tokio::{fs, io};

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentType {
    Movies,
    Shows,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Library {
    pub name: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub folders: Vec<PathBuf>,

    #[serde(skip)]
    pub movies: Vec<Movie>,
}

impl Library {
    pub async fn scan(&mut self, cache: &Cache, skip_cache: bool) -> io::Result<()> {
        let tmdb_client = TMDb::new(dotenv::var("TMDB_KEY").expect("Failed to get TMDb API key"));

        for folder in &self.folders {
            let mut entries = fs::read_dir(folder).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let movie = movie::fetch_info(path, &cache, skip_cache, &tmdb_client).await;
                cache.store_movie(&movie).await?;

                self.movies.push(movie);
            }
        }

        Ok(())
    }
}
