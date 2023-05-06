use super::metadata_service::{CreationParam, MetadataResult, MetadataService};
use crate::config::Config;
use crate::data_node_client::DataNodeClient;
use crate::storage_types::commit_types::block::Block;
use crate::storage_types::commit_types::commit::Commits;
use crate::storage_types::commit_types::merkle_tree::MerkleTree;
use crate::storage_types::commit_types::sequence::Sequence;
use crate::storage_types::large_file::LargeFile;
use crate::storage_types::object::{Object, ObjectVariant};
use crate::storage_types::small_file::SmallFile;
use async_trait::async_trait;
use redis::{JsonCommands, RedisResult};
use shared::main_server_error::MetadataError;
use smallvec::SmallVec;
use std::path::Path;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct MetaServiceRedis {
    storage: redis::Client,
    data_node_client: DataNodeClient,
    config: Config,
}

impl MetaServiceRedis {
    pub async fn new(
        redis: redis::Client,
        data_node_client: DataNodeClient,
        config: Config,
    ) -> Self {
        Self {
            storage: redis,
            data_node_client,
            config,
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
        let response = self.data_node_client.create_blocks(1).await?;
        let object = Object::new(
            params.path.as_ref().to_string_lossy().into(),
            params.size,
            ObjectVariant::SmallFile(SmallFile {
                commits: Commits::Sequence(Sequence {
                    seq: vec![Block {
                        id: Uuid::from_slice(response.blocks[0].block_id.as_slice()).unwrap(/*Never panic*/),
                        part: 0,
                        dst: response.endpoint,
                        replicas: vec![],
                        checksum: 0u32,
                    }],
                }),
            }),
        );

        let mut connection = self.storage.get_connection().unwrap();
        let _: RedisResult<bool> = connection.json_set(
            params.path.as_ref().to_string_lossy().to_string(),
            "$",
            &object,
        );

        Ok(object)
    }

    async fn create_large_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        let block_count = params.size / self.config.block_size + 1;
        let mut response = self.data_node_client.create_blocks(block_count).await?;
        response.blocks.sort_by(|a, b| a.part.cmp(&b.part));

        let blocks = response
            .blocks
            .iter()
            .map(|el| Block {
                id: Uuid::from_slice(el.block_id.as_slice()).unwrap(/*Never panic*/),
                part: el.part as usize,
                dst: response.endpoint.clone(),
                replicas: vec![],
                checksum: 0u32,
            })
            .collect();

        let object = Object::new(
            params.path.as_ref().to_string_lossy().into(),
            params.size,
            ObjectVariant::LargeFile(LargeFile {
                parts: MerkleTree::build(blocks),
            }),
        );

        let mut connection = self.storage.get_connection().unwrap();
        let _: RedisResult<bool> = connection.json_set(
            params.path.as_ref().to_string_lossy().to_string(),
            "$",
            &object,
        );

        Ok(object)
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
