use clap::Parser;
use serde::Deserialize;
use std::path::Path;
use tokio::io::AsyncReadExt;

#[derive(Deserialize, Parser)]
pub struct Config {
    /// Address of main server
    #[arg(short, long)]
    pub(crate) main_server_address: String,
    /// Address of this instance
    #[arg(short, long)]
    pub(crate) self_address: String,
    /// Port
    #[arg(short, long, default_value_t = 40000)]
    pub(crate) port: u16,
    /// Block size
    #[arg(short, long)]
    pub(crate) block_size: usize,
    /// Volume of disk space to use in KB. If not set service will use all disk space
    #[arg(short, long)]
    pub(crate) disk_space: Option<u64>,
    /// Name of directory where will placed blocks
    #[arg(short, long)]
    pub(crate) working_directory: String,
    /// Buffer size for stream reading
    #[arg(short, long)]
    pub(crate) read_buffer: usize,
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
