use crate::{
    cache::Cache,
    movie::{self, Movie},
};
use log::error;
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
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub name: String,
    #[serde(rename = "type")]
    pub content_type: ContentType,
    pub folders: Vec<PathBuf>,
    #[serde(default)]
    pub skip_cache: bool,

    #[serde(skip)]
    pub movies: Vec<Movie>,
}

impl Library {
    pub async fn scan(&mut self, cache: &Cache) -> io::Result<()> {
        let tmdb = TMDb::new(dotenv::var("TMDB_KEY").expect("Failed to get TMDb API key"));

        for folder in &self.folders {
            let mut entries = fs::read_dir(folder).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let (cached, mut movie) = match movie::fetch_info(path, &cache, self.skip_cache, &tmdb).await {
                    Ok(movie) => movie,
                    Err(e) => {
                        error!("Failed to fetch movie info: {:#?}", e);
                        continue;
                    }
                };

                if !cached {
                    cache.store_movie(&mut movie).await?;
                }

                self.movies.push(movie);
            }
        }

        Ok(())
    }
}
