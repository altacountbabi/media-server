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

pub fn fileprefix(path: impl Into<PathBuf>) -> String {
    Into::<PathBuf>::into(path)
        .file_prefix()
        .expect("Failed to get file prefix")
        .to_string_lossy()
        .to_string()
}

pub async fn create_dir_if_not_exist(path: impl Into<PathBuf>) -> io::Result<()> {
    let path = path.into();
    if !path.exists() {
        create_dir_all(&path).await
    } else {
        Ok(())
    }
}
