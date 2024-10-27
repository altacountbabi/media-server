use std::path::PathBuf;

use regex::Regex;

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
