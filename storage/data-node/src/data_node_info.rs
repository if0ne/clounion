use crate::config::Config;
use crate::disk_stats::DiskStats;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use sysinfo::{DiskExt, SystemExt};
use tokio::io::SeekFrom;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};

#[derive(Debug)]
pub struct DataNodeInfo {
    pub port: u16,
    pub self_address: String,
    pub(crate) working_directory: PathBuf,
    pub(crate) block_size: usize,
    pub(crate) read_buffer: usize,
    pub(crate) total_space: u64,
    pub(crate) disks: Vec<DiskStats>,
}

impl DataNodeInfo {
    pub async fn new(config: Config) -> Self {
        let path = format!("{}_{}", config.working_directory, config.block_size);
        let working_directory = PathBuf::from(&path);
        let disks = Self::get_disks(config.block_size, &working_directory);
        let total_space = disks
            .iter()
            .fold(0, |space, disk| space + disk.available_space);
        let used_space = Self::get_used_space(&disks).await;

        let status = Self::check_config(&config, &path).await;
        if let Ok(status) = status {
            if !status && used_space != 0 {
                panic!("Wrong block size. Please recover old block size or clean up all working directories.");
            }
        } else if used_space != 0 {
            panic!("Block size was missed and file system isn't empty. Please recover old block size or clean up all working directories.");
        }

        if Self::save_state(&config, &path).await.is_err() {
            tracing::warn!("Can't save the state. It can lead to memory inconsistency.");
        }

        tracing::debug!(
            "Working directory: {} ",
            working_directory.to_string_lossy()
        );
        tracing::debug!(
            "Total space: {} bytes | {} Kb | {} Mb | {} Gb",
            total_space,
            total_space / 1024,
            total_space / 1024 / 1024,
            total_space / 1024 / 1024 / 1024
        );
        tracing::debug!(
            "Used space: {} bytes | {} Kb | {} Mb | {} Gb",
            used_space,
            used_space / 1024,
            used_space / 1024 / 1024,
            used_space / 1024 / 1024 / 1024
        );
        tracing::debug!(
            "Block size: {} bytes | {} Kb | {} Mb",
            config.block_size,
            config.block_size / 1024,
            config.block_size / 1024 / 1024
        );

        Self {
            port: config.port,
            self_address: config.self_address,
            read_buffer: config.read_buffer,
            working_directory,
            block_size: config.block_size,
            total_space,
            disks,
        }
    }

    pub(crate) async fn get_disk_for_new_block(&self) -> Result<&DiskStats, ()> {
        for disk in &self.disks {
            let mut writer = disk.used_space.write().await;
            if *writer + (self.block_size as u64) <= self.total_space {
                *writer += self.block_size as u64;
                return Ok(disk);
            }
        }

        Err(())
    }

    pub(crate) async fn found_block(&self, uuid: impl AsRef<Path>) -> Result<PathBuf, ()> {
        for disk in &self.disks {
            let path = disk.mount.join(&self.working_directory).join(&uuid);
            if path.exists() {
                return Ok(path);
            }
        }

        Err(())
    }

    async fn save_state(config: &Config, suffix: &str) -> std::io::Result<()> {
        let mut hasher = DefaultHasher::new();
        ((config.block_size as u64 * 2) >> 6).hash(&mut hasher);

        let hash = hasher.finish();
        let path = format!(".data_node_info_{}", suffix);
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&path)
            .await?;
        file.seek(SeekFrom::Start(0)).await?;
        file.write_u64(hash).await?;

        Ok(())
    }

    async fn check_config(config: &Config, suffix: &str) -> std::io::Result<bool> {
        let mut hasher = DefaultHasher::new();
        ((config.block_size as u64 * 2) >> 6).hash(&mut hasher);

        let hash = hasher.finish();
        let path = format!(".data_node_info_{}", suffix);
        let mut file = tokio::fs::OpenOptions::new().read(true).open(&path).await?;
        let read = file.read_u64().await?;

        Ok(hash == read)
    }

    async fn get_used_space(disks: &[DiskStats]) -> u64 {
        let mut used_space = 0;
        for disk in disks {
            used_space += *disk.used_space.read().await
        }

        used_space
    }

    fn get_disks<P: AsRef<Path>>(block_size: usize, working_directory: P) -> Vec<DiskStats> {
        let mut system = sysinfo::System::new_all();
        system.refresh_all();
        system.sort_disks_by(|l_disk, r_disk| r_disk.available_space().cmp(&l_disk.total_space()));

        system
            .disks()
            .iter()
            .filter_map(|disk| {
                DiskStats::new(
                    disk.available_space(),
                    block_size,
                    disk.mount_point(),
                    working_directory.as_ref(),
                )
            })
            .collect()
    }
}
