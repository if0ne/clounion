use super::metadata_service::{CreationParam, MetadataResult, MetadataService};
use crate::data_node_client::DataNodeClient;
use crate::storage_types::commit_types::block::Block;
use crate::storage_types::commit_types::commit::Commits;
use crate::storage_types::commit_types::sequence::Sequence;
use crate::storage_types::object::{Object, ObjectVariant};
use crate::storage_types::small_file::SmallFile;
use async_trait::async_trait;
use redis::JsonCommands;
use shared::main_server_error::MetadataError;
use smallvec::SmallVec;
use std::path::Path;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct MetaServiceRedis {
    storage: RwLock<redis::Client>,
    data_node_client: DataNodeClient,
}

impl MetaServiceRedis {
    pub async fn new(redis: redis::Client, data_node_client: DataNodeClient) -> Self {
        Self {
            storage: RwLock::new(redis),
            data_node_client,
        }
    }
}

#[async_trait]
impl MetadataService for MetaServiceRedis {
    type Dst = String;
    type Hash = u32;

    async fn create_small_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        let block = self.data_node_client.create_blocks(1).await?;
        let object = Ok(Object::new(
            params.path.as_ref().to_string_lossy().into(),
            ObjectVariant::SmallFile(SmallFile {
                commits: Commits::Sequence(Sequence {
                    seq: vec![Block {
                        id: Uuid::from_slice(block.blocks[0].block_id.as_slice()).unwrap(/*Never panic*/),
                        part: 0,
                        dst: block.endpoint,
                        replicas: SmallVec::new(),
                        checksum: 0u32,
                    }],
                }),
            }),
        ));

        self.storage
            .write()
            .json_set(
                params.path.as_ref().to_string_lossy().to_string(),
                "",
                &object,
            )
            .unwrap();

        object
    }

    async fn create_large_file<P: AsRef<Path> + Send>(
        &self,
        path: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
    }

    async fn get_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
    }

    async fn get_small_file_last_version<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
    }

    async fn add_commit_to_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
    }

    async fn get_large_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
    }

    async fn delete_object<P: AsRef<Path> + Send>(&self, path: P) -> MetadataResult<()> {
        todo!()
    }
}
