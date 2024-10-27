use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::{fs, io};

use crate::movie::{self, Movie};

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
    pub async fn scan(&mut self, omdb_api_key: impl Into<String>) -> io::Result<()> {
        let omdb_api_key: &str = &omdb_api_key.into();

        for folder in &self.folders {
            let mut entries = fs::read_dir(folder).await?;

            while let Some(entry) = entries.next_entry().await? {
                let path = entry.path();
                let movie = movie::fetch_info(path, omdb_api_key);

                self.movies.push(movie.await);
            }
        }

        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ContentType {
    Movies,
    Shows,
}
