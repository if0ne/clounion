use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Parser)]
pub struct Config {
    /// Address of this instance
    #[arg(short, long)]
    pub(crate) self_address: String,
    /// Port
    #[arg(short, long, default_value_t = 40000)]
    pub(crate) port: u16,
    /// Block size
    #[arg(short, long)]
    pub(crate) block_size: usize,
    /// Maximum size of small file in KB
    #[arg(short, long)]
    pub(crate) max_small_file_size: usize,
    /// Connection string for database
    #[arg(short, long)]
    pub(crate) database_connection: String,
}

impl Config {
    pub async fn try_from_file<P: AsRef<Path>>(path: P) -> Self {
        Self::from_file(path)
            .await
            .unwrap_or_else(|_| Self::parse())
    }

    pub async fn from_file<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let mut config_file = tokio::fs::OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)
            .await?;
        let mut buffer = String::new();
        config_file.read_to_string(&mut buffer).await?;
        let config: Config = toml::from_str(&buffer).unwrap();

        Ok(config)
    }
}
