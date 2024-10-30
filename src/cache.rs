use crate::{movie::Movie, utils::create_dir_if_not_exist};
use image::ImageFormat;
use log::{error, trace};
use std::path::{Path, PathBuf};
use tokio::{fs, io};

const IMAGE_FORMAT: &str = "webp";

pub struct Cache {
    pub cache_path: PathBuf,
}

impl Cache {
    pub async fn new(data_path: &Path) -> io::Result<Self> {
        let cache_path = data_path.join("cache");
        create_dir_if_not_exist(&cache_path).await?;

        Ok(Self { cache_path })
    }

    pub async fn cache_image(&self, url: impl Into<String>, path: impl Into<PathBuf>) {
        let url: String = url.into();
        let path: PathBuf = path.into();

        let response = match reqwest::get(&url).await {
            Ok(response) => response,
            Err(err) => {
                error!("Failed to cache image '{url}' to '{}': {err}", path.display());
                return;
            }
        };

        let bytes = match response.bytes().await {
            Ok(bytes) => bytes,
            Err(err) => {
                error!("Failed to read response body when caching image '{url}':\n{err}");
                return;
            }
        };

        let image = match image::load_from_memory(&bytes) {
            Ok(image) => image,
            Err(err) => {
                error!("Failed to decode image '{url}':\n{err}");
                return;
            }
        };

        if let Err(err) = image.save_with_format(&path, ImageFormat::from_extension(IMAGE_FORMAT).unwrap_or(ImageFormat::Png)) {
            error!("Failed to write image to '{}': {err}", path.display());
        }
    }

    pub async fn store_movie(&self, movie: &mut Movie) -> io::Result<()> {
        let path = self.cache_path.join(&movie.path);

        create_dir_if_not_exist(&path).await?;

        {
            trace!("Caching backdrop for '{}'", &movie.metadata.title);
            let path = path.join("backdrop").with_extension(IMAGE_FORMAT);
            self.cache_image(&movie.metadata.backdrop_path.replace("original", "w500"), &path)
                .await;
            movie.metadata.backdrop_path = path.to_string_lossy().to_string();
        }

        {
            trace!("Caching poster for '{}'", &movie.metadata.title);
            let path = path.join("poster").with_extension(IMAGE_FORMAT);
            self.cache_image(&movie.metadata.poster_path.replace("original", "w500"), &path)
                .await;
            movie.metadata.poster_path = path.to_string_lossy().to_string();
        }

        let serialized = match bincode::serialize(&movie) {
            Ok(serialized) => serialized,
            Err(err) => {
                error!("Failed to serialize movie: {err}");
                return Err(io::ErrorKind::Other.into());
            }
        };

        fs::write(path.join("metadata.bin"), serialized).await
    }

    pub async fn get_movie(&self, name: impl Into<String>) -> io::Result<Option<Movie>> {
        let name: String = name.into();
        let path = self.cache_path.join("movies").join(name).join("metadata.bin");

        if !path.exists() {
            return Ok(None);
        }

        let bytes = match fs::read(&path).await {
            Ok(bytes) => bytes,
            Err(err) => {
                error!("Failed to read movie metadata from cache: {err}");
                return Ok(None);
            }
        };
        let deserialized: Movie = match bincode::deserialize(&bytes) {
            Ok(movie) => movie,
            Err(err) => {
                error!("Failed to deserialize movie metadata from cache: {err}");
                return Ok(None);
            }
        };

        Ok(Some(deserialized))
    }
}
