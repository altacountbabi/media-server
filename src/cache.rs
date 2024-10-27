use crate::{
    movie::Movie,
    utils::{create_dir_if_not_exist, fileprefix},
};
use std::path::{Path, PathBuf};
use tokio::{fs, io};

pub struct Cache {
    pub cache_path: PathBuf,
}

impl Cache {
    pub async fn new(data_path: &Path) -> io::Result<Self> {
        let cache_path = data_path.join("cache").to_path_buf();
        create_dir_if_not_exist(&cache_path).await?;

        Ok(Self { cache_path })
    }

    pub async fn store_movie(&self, movie: &Movie) -> io::Result<()> {
        let path = self
            .cache_path
            .join(&movie.path.parent().expect("Failed to get parent of movie path"));

        create_dir_if_not_exist(&path).await?;

        fs::write(
            path.join(fileprefix(&movie.path)),
            bincode::serialize(&movie).expect("Failed to serialize movie"),
        )
        .await
    }

    pub async fn get_movie(&self, name: impl Into<String>) -> io::Result<Option<Movie>> {
        let name: String = name.into();
        let path = self.cache_path.join("movies").join(name);

        if path.exists() {
            Ok(Some(
                bincode::deserialize(&fs::read(path).await?).expect("Failed to deserialize movie from cache"),
            ))
        } else {
            Ok(None)
        }
    }
}
