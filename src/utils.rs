use regex::Regex;
use std::path::PathBuf;
use tokio::{fs::create_dir_all, io};

pub fn re(regex: impl Into<String>) -> Regex {
    Regex::new(&regex.into()).unwrap()
}

pub fn filename(path: impl Into<PathBuf>) -> String {
    Into::<PathBuf>::into(path)
        .file_name()
        .expect("Failed to get file name")
        .to_string_lossy()
        .to_string()
}

#[allow(dead_code)]
pub fn file_ext(path: impl Into<PathBuf>) -> String {
    Into::<PathBuf>::into(path)
        .file_name()
        .expect("Failed to get file name")
        .to_string_lossy()
        .split('.')
        .last()
        .expect("Failed to get file extension")
        .to_string()
}

pub async fn create_dir_if_not_exist(path: impl Into<PathBuf>) -> io::Result<()> {
    let path = path.into();
    if path.exists() {
        Ok(())
    } else {
        create_dir_all(&path).await
    }
}
