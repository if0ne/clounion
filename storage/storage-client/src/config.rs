use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

#[derive(Clone, Deserialize, Parser)]
pub struct Config {
    /// Address of main server
    #[arg(short, long)]
    pub(crate) main_server_address: String,
    #[arg(short, long)]
    pub(crate) read_buffer: usize,
    /// Block size
    #[arg(short, long)]
    pub(crate) block_size: usize,
    /// Идентификатор пользователя
    pub(crate) client_id: Uuid,
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

    pub fn get_main_server_addr(&self) -> &str {
        &self.main_server_address
    }
}
