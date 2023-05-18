use crate::data_node_info::DataNodeInfo;
use futures::TryFutureExt;
use shared::data_node_error::DataNodeError;
use std::io::SeekFrom;
use std::ops::Range;
use std::path::{Path, PathBuf};
use tokio::fs::OpenOptions;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufReader, BufWriter};
use uuid::Uuid;

pub struct BlockStorage {
    data_node_info: DataNodeInfo,
}

impl BlockStorage {
    pub async fn new(data_node_info: DataNodeInfo) -> std::io::Result<Self> {
        for disk in &data_node_info.disks {
            let path = disk.mount.join(&data_node_info.working_directory);
            if !path.exists() {
                tokio::fs::create_dir(path).await?;
            }
        }

        Ok(Self { data_node_info })
    }

    pub fn get_endpoint(&self) -> String {
        self.data_node_info.get_endpoint()
    }

    pub async fn create_block(
        &self,
        part: usize,
        uuid: Uuid,
    ) -> Result<(usize, Uuid), DataNodeError> {
        let disk = self.data_node_info.get_disk_for_new_block().await?;

        let _ = OpenOptions::new()
            .write(true)
            .read(false)
            .create(true)
            .open(
                disk.mount
                    .join(&self.data_node_info.working_directory)
                    .join(format!("{}_{}", uuid.as_u128(), part)),
            )
            .await
            .map_err(|err| DataNodeError::CreateBlockError(err.to_string()))?;

        Ok((part, uuid))
    }

    /// Лучше возврощать поток чтения, чтобы напрямую передавать байты
    pub async fn read_block<P: AsRef<Path>>(
        &self,
        path: P,
        bytes: Range<usize>,
    ) -> Result<Vec<u8>, DataNodeError> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(path)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;
        let mut reader = BufReader::new(file);
        let mut buffer = vec![0; bytes.len()];
        reader
            .seek(SeekFrom::Start(bytes.start as u64))
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))
            .await?;
        reader
            .read_exact(&mut buffer)
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;
        reader
            .flush()
            .await
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?;

        Ok(buffer)
    }

    pub async fn update_block(
        &self,
        block_id: Uuid,
        part: usize,
        bytes: Range<usize>,
        data: &[u8],
    ) -> Result<(), DataNodeError> {
        let (path, size) = self.get_block_info(block_id, part).await?;

        if data.len() > self.data_node_info.block_size {
            return Err(DataNodeError::BlockOverflow(
                self.data_node_info.block_size,
                data.len(),
            ));
        }

        if data.len() > bytes.len() {
            return Err(DataNodeError::BlockOverflow(bytes.len(), data.len()));
        }

        if data.len() > (bytes.start..self.data_node_info.block_size).len() {
            return Err(DataNodeError::BlockOverflow(
                (bytes.start..size).len(),
                data.len(),
            ));
        }

        let file = OpenOptions::new()
            .write(true)
            .read(false)
            .open(&path)
            .await
            .map_err(|_| DataNodeError::UpdateBlockError(block_id.to_string()))?;

        let mut writer = BufWriter::new(file);

        writer
            .seek(SeekFrom::Start(bytes.start as u64))
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;
        writer
            .write_all(data)
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;
        writer
            .flush()
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;

        Ok(())
    }

    pub async fn delete_block(&self, block_id: Uuid, part: usize) -> Result<(), DataNodeError> {
        let path = self
            .data_node_info
            .found_block(format!("{}_{}", block_id.as_u128(), part))
            .await?;
        tokio::fs::remove_file(path)
            .await
            .map_err(|_| DataNodeError::DeleteBlockError(block_id.to_string()))
    }

    pub async fn get_block_info(
        &self,
        block_id: Uuid,
        part: usize,
    ) -> Result<(PathBuf, usize), DataNodeError> {
        let path = self
            .data_node_info
            .found_block(format!("{}_{}", block_id.as_u128(), part))
            .await?;
        let buffer_len = path
            .metadata()
            .map_err(|err| DataNodeError::ReadBlockError(err.to_string()))?
            .len() as usize;

        Ok((path, buffer_len))
    }

    pub fn get_data_node_info(&self) -> &DataNodeInfo {
        &self.data_node_info
    }

    pub async fn get_checksum(&self, block_id: Uuid, part: usize) -> Result<u32, DataNodeError> {
        let (path, _) = self.get_block_info(block_id, part).await?;

        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .open(&path)
            .await
            .map_err(|_| DataNodeError::UpdateBlockError(block_id.to_string()))?;

        let mut reader = BufReader::new(file);
        let _ = reader
            .seek(SeekFrom::Start(0))
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;

        let mut buffer = vec![];
        let _ = reader
            .read_to_end(&mut buffer)
            .await
            .map_err(|err| DataNodeError::UpdateBlockError(err.to_string()))?;

        Ok(crc32fast::hash(&buffer))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;

    #[tokio::test]
    async fn test_block_storage_crud() {
        let message = b"Hello, Pavel";

        let data_node_info = DataNodeInfo::new(Config {
            main_server_address: "http://[::1]:8000".to_string(),
            self_address: "http://[::1]".to_string(),
            port: 40000,
            block_size: 32,
            max_small_file_size: 16,
            disk_space: None,
            working_directory: "test_dir".to_string(),
            read_buffer: 8,
        })
            .await;
        let buffer_size = data_node_info.io_buffer;

        let data_node = BlockStorage::new(data_node_info).await.unwrap();
        let (part, uuid) = data_node.create_block(0, Uuid::new_v4()).await.unwrap();

        data_node.update_block(uuid, part, 0..message.len(), message).await.unwrap();
        let (path, len) = data_node.get_block_info(uuid, part).await.unwrap();
        let chunk_count = len / buffer_size;
        let last_chunk = len - chunk_count * buffer_size;

        let mut read_message = vec![];
        for i in 0..(chunk_count + 1) {
            let bytes = if i == chunk_count {
                if last_chunk == 0 {
                    break;
                }
                (i * buffer_size)..(i * buffer_size + last_chunk)
            } else {
                (i * buffer_size)..((i + 1) * buffer_size)
            };
            let read = data_node.read_block(&path, bytes).await.unwrap();
            read_message = [read_message, read].concat();
        }

        assert_eq!(message.as_slice(), read_message.as_slice());
        data_node.delete_block(uuid, part).await.unwrap();
    }

    #[tokio::test]
    async fn test_multi_read_access() {
        let message = std::iter::repeat(*b"Hello, Pavel ")
            .take(4096)
            .flatten()
            .collect::<Vec<_>>();
        assert_eq!(message.len(), 13 * 4096);

        let data_node_info = DataNodeInfo::new(Config {
            main_server_address: "http://[::1]:8000".to_string(),
            self_address: "http://[::1]".to_string(),
            port: 40000,
            block_size: 65536,
            max_small_file_size: 65536,
            disk_space: None,
            working_directory: "test_dir".to_string(),
            read_buffer: 1000,
        })
            .await;
        let buffer_size = data_node_info.io_buffer;

        let data_node = BlockStorage::new(data_node_info).await.unwrap();
        let (part, uuid) = data_node.create_block(0, Uuid::new_v4()).await.unwrap();

        data_node.update_block(uuid, part, 0..message.len(), &message).await.unwrap();
        let (path, len) = data_node.get_block_info(uuid, part).await.unwrap();
        let chunk_count = len / buffer_size;
        let last_chunk = len - chunk_count * buffer_size;
        let mut futures = vec![];
        for _ in 0..100 {
            futures.push(async {
                let mut read_message = vec![];
                for i in 0..(chunk_count + 1) {
                    let bytes = if i == chunk_count {
                        if last_chunk == 0 {
                            break;
                        }
                        (i * buffer_size)..(i * buffer_size + last_chunk)
                    } else {
                        (i * buffer_size)..((i + 1) * buffer_size)
                    };
                    let read = data_node.read_block(&path, bytes).await.unwrap();
                    read_message = [read_message, read].concat();
                }
                read_message
            })
        }
        let results = futures::future::join_all(futures).await;
        for i in results {
            assert_eq!(message.as_slice(), i.as_slice());
        }

        data_node.delete_block(uuid, part).await.unwrap();
    }
}

#[cfg(any(test, bench))]
impl Drop for BlockStorage {
    fn drop(&mut self) {
        for disk in &self.data_node_info.disks {
            let _ =
                std::fs::remove_dir_all(disk.mount.join(&self.data_node_info.working_directory));
        }
    }
}
