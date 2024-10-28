#![feature(path_file_prefix)]

use cache::Cache;
use clap::Parser;
use log::{error, info, trace};
use std::{path::PathBuf, process};
use tokio::io;

mod cache;
mod config;
mod library;
mod movie;
mod password;
mod utils;

#[derive(Debug, Parser)]
struct Cli {
    #[cfg(debug_assertions)]
    #[arg(short = 'd', long = "data", required = false, default_value = "data")]
    data_path: PathBuf,

    #[cfg(not(debug_assertions))]
    #[arg(short = 'd', long = "data", required = true)]
    data_path: PathBuf,
}

#[tokio::main]
async fn main() -> io::Result<()> {
    colog::init();

    let Cli { data_path } = Cli::parse();

    if !data_path.exists() {
        error!("Data directory does not exist: {}", data_path.display());
        process::exit(1);
    }

    let mut config = config::eval_config(data_path.join("config")).expect("Failed to evaluate config file");
    let cache = Cache::new(&config.data_dir).await?;

    for library in &mut config.libraries {
        info!("Scanning library: {}", library.name);
        library.scan(&cache, false).await?;
        trace!("Found movies:\n{:#?}", library.movies);
    }

    Ok(())
}
