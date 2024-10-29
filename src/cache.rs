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
        let cache_path = data_path.join("cache").to_path_buf();
        create_dir_if_not_exist(&cache_path).await?;

        Ok(Self { cache_path })
    }

    pub async fn cache_image(&self, url: impl Into<String>, path: impl Into<PathBuf>) {
        let url: String = url.into();
        let path: PathBuf = path.into();

        match reqwest::get(&url).await {
            Ok(response) => {
                let bytes = response.bytes().await.expect("Failed to read response");
                let image = image::load_from_memory(&bytes).expect("Failed to load image");
                match image.save_with_format(&path, ImageFormat::from_extension(IMAGE_FORMAT).unwrap_or(ImageFormat::Png)) {
                    Ok(()) => (),
                    Err(e) => {
                        error!("Failed to write image to '{}': {:#?}", path.display(), e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to cache image '{url}' to '{}': {:#?}", path.display(), e);
                return;
            }
        }
    }

    pub async fn store_movie(&self, movie: &mut Movie) -> io::Result<()> {
        let path = self.cache_path.join(&movie.path);

        create_dir_if_not_exist(&path).await?;

        {
            trace!("Caching backdrop for '{}'", &movie.metadata.title);
            let path = path.join("backdrop").with_extension(IMAGE_FORMAT);
            self.cache_image(&movie.metadata.backdrop_path, &path).await;
            movie.metadata.backdrop_path = path.to_string_lossy().to_string();
        }

        {
            trace!("Caching poster for '{}'", &movie.metadata.title);
            let path = path.join("poster").with_extension(IMAGE_FORMAT);
            self.cache_image(&movie.metadata.poster_path, &path).await;
            movie.metadata.poster_path = path.to_string_lossy().to_string();
        }

        for production_company in &mut movie.metadata.production_companies {
            trace!(
                "Caching production company '{}' logo for '{}'",
                production_company.name,
                &movie.metadata.title
            );

            let path = path.join(format!(
                "{}-{}-logo.{}",
                production_company.id, production_company.name, IMAGE_FORMAT
            ));
            self.cache_image(&production_company.logo_path, &path).await;

            production_company.logo_path = path.to_string_lossy().to_string();
        }

        fs::write(
            path.join("metadata.bin"),
            bincode::serialize(&movie).expect("Failed to serialize movie"),
        )
        .await?;

        Ok(())
    }

    pub async fn get_movie(&self, name: impl Into<String>) -> io::Result<Option<Movie>> {
        let name: String = name.into();
        let path = self.cache_path.join("movies").join(name);

        if !path.exists() {
            return Ok(None);
        }

        Ok(Some(
            bincode::deserialize(&fs::read(path.join("metadata.bin")).await?).expect("Failed to deserialize movie from cache"),
        ))
    }
}
