use crate::block_storage::BlockStorage;
use crate::data_node_info::DataNodeInfo;
use shared::data_node_error::DataNodeError;
use std::ops::Range;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub struct BlockStorageService {
    block_storage: BlockStorage,
}

impl BlockStorageService {
    pub async fn new(data_node_info: DataNodeInfo) -> std::io::Result<Self> {
        let block_storage = BlockStorage::new(data_node_info).await?;

        Ok(Self { block_storage })
    }

    pub fn get_endpoint(&self) -> String {
        self.block_storage.get_endpoint()
    }

    pub async fn create_blocks(&self, count: usize) -> Result<Vec<(usize, Uuid)>, DataNodeError> {
        let uuid = Uuid::new_v4();

        let tasks =
            (0..count).map(|id| async move { self.block_storage.create_block(id, uuid).await });
        let tasks = futures::future::join_all(tasks).await;

        if !tasks.iter().all(|block| block.is_ok()) {
            for task in tasks {
                match task {
                    Ok((part, uuid)) => {
                        if self.delete_block(uuid, part).await.is_err() {
                            tracing::error!("Can not clean up block with id: {:x}", uuid);
                        }
                    }
                    Err(err) => {
                        tracing::error!("{}", err);
                    }
                }
            }

            return Err(DataNodeError::CreateBlocksError(format!(
                "Can not create {count} blocks"
            )));
        }

        Ok(tasks
            .into_iter()
            .map(|el| el.unwrap(/*Safe because all items was checked for error*/))
            .collect())
    }

    pub async fn read_block(
        &self,
        block_id: Uuid,
        part: usize,
        tx: Sender<Result<Vec<u8>, DataNodeError>>,
    ) -> Result<(), DataNodeError> {
        let (path, file_size) = self.block_storage.get_block_info(block_id, part).await?;

        let buffer_size = self.block_storage.get_data_node_info().io_buffer;

        let chunk_count = file_size / buffer_size;
        let last_chunk = file_size - chunk_count * buffer_size;

        for i in 0..(chunk_count + 1) {
            let bytes = if i == chunk_count {
                if last_chunk == 0 {
                    break;
                }

                (i * buffer_size)..(i * buffer_size + last_chunk)
            } else {
                (i * buffer_size)..((i + 1) * buffer_size)
            };

            let read = self.block_storage.read_block(&path, bytes).await;

            match tx.send(read).await {
                Ok(_) => {
                    tracing::debug!("Sent chunk number {} of {}", i, block_id);
                }
                Err(_) => {
                    tracing::error!("Read stream for {} was dropped", block_id);
                    break;
                }
            }
        }

        Ok(())
    }

    pub async fn update_block(
        &self,
        block_id: Uuid,
        part: usize,
        range: Range<usize>,
        data: &[u8],
    ) -> Result<(), DataNodeError> {
        self.block_storage
            .update_block(block_id, part, range, data)
            .await
    }

    pub async fn get_block_checksum(
        &self,
        block_id: Uuid,
        part: usize,
    ) -> Result<u32, DataNodeError> {
        self.block_storage.get_checksum(block_id, part).await
    }

    pub async fn delete_block(&self, block_id: Uuid, part: usize) -> Result<(), DataNodeError> {
        self.block_storage.delete_block(block_id, part).await
    }
}
