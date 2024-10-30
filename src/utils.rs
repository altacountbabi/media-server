use log::error;
use regex::Regex;
use std::path::PathBuf;
use tokio::{fs::create_dir_all, io};

pub fn re(regex: impl Into<String>) -> Regex {
    Regex::new(&regex.into()).unwrap()
}

pub fn filename(path: impl Into<PathBuf>) -> io::Result<String> {
    let path: PathBuf = path.into();

    if let Some(name) = path.file_name() {
        Ok(name.to_string_lossy().to_string())
    } else {
        error!("Failed to get file name from: {}", path.display());
        Err(io::ErrorKind::Other.into())
    }
}

#[allow(dead_code)]
pub fn file_ext(path: impl Into<PathBuf>) -> io::Result<String> {
    let path: PathBuf = path.into();

    if let Some(name) = path.file_name() {
        if let Some(ext) = name.to_string_lossy().split('.').last() {
            Ok(ext.to_string())
        } else {
            error!("Failed to get file extension from: {}", path.display());
            Err(io::ErrorKind::Other.into())
        }
    } else {
        error!("Failed to get file name from: {}", path.display());
        Err(io::ErrorKind::Other.into())
    }
}

pub async fn create_dir_if_not_exist(path: impl Into<PathBuf>) -> io::Result<()> {
    let path: PathBuf = path.into();
    if path.exists() {
        Ok(())
    } else {
        create_dir_all(&path).await
    }
}
