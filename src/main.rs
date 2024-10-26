use log::info;
use tokio::io;

mod config;
mod library;
mod movie;
mod password;

#[tokio::main]
async fn main() -> io::Result<()> {
    colog::init();

    let mut config = config::eval_config().expect("Failed to evaluate config file");

    for library in &mut config.libraries {
        info!("Scanning library: {}", library.name);
        library.scan(&config.api_keys.omdb).await?;
    }

    dbg!(&config.libraries);

    Ok(())
}