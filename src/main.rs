#![feature(path_file_prefix, let_chains)]

use cache::Cache;
use clap::Parser;
use log::{debug, error, info, LevelFilter};
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
    logger_init();

    let Cli { data_path } = Cli::parse();

    if !data_path.exists() {
        error!("Data directory does not exist: {}", data_path.display());
        process::exit(1);
    }

    let mut config = config::read(&data_path.join("config"));
    let cache = match Cache::new(&config.data_dir).await {
        Ok(cache) => cache,
        Err(err) => {
            error!("Failed to initialize cache: {err}");
            process::exit(1);
        }
    };

    for library in &mut config.libraries {
        info!("Scanning library: {}", library.name);
        library.scan(&cache).await?;
        debug!("Found movies:\n{:#?}", library.movies);
    }

    Ok(())
}

fn logger_init() {
    let mut logger = colog::default_builder();

    if cfg!(debug_assertions) {
        logger.filter_level(LevelFilter::Trace);
    } else {
        logger.filter_level(LevelFilter::Info);
    }

    logger.filter_module("reqwest", LevelFilter::Warn).init();
}
