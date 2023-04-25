use tokio::sync::mpsc::Sender;
use uuid::Uuid;

pub struct BlockStorageServiceImpl {
}

impl BlockStorageServiceImpl {
    pub fn new() -> Self {
        Self {
        }
    }

    pub async fn create_block(&self) -> Result<Uuid, ()> {
        todo!()
    }

    pub async fn read_block(
        &self,
        block_id: &[u8],
        tx: Sender<Result<Vec<u8>, ()>>,
    ) -> Result<(), ()> {
        todo!()
    }

    pub async fn update_block(&self, block_id: &[u8], data: &[u8]) -> Result<(), ()> {
        todo!()
    }

    pub async fn delete_block(&self, block_id: &[u8]) -> Result<(), ()> {
        todo!()
    }
}