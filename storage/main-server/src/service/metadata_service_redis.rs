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
use redis::{AsyncCommands, JsonAsyncCommands, RedisResult};
use shared::main_server_error::MetadataError;
use std::path::Path;
use std::sync::Arc;
use uuid::Uuid;

pub struct MetaServiceRedis {
    storage: redis::Client,
    data_node_client: Arc<DataNodeClient>,
    config: Config,
}

impl MetaServiceRedis {
    pub async fn new(
        redis: redis::Client,
        data_node_client: Arc<DataNodeClient>,
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

    async fn create_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst>> {
        let response = self.data_node_client.create_blocks(1).await?;
        let object = Object::new(
            params.path.as_ref().to_string_lossy().into(),
            params.size,
            params.user_id,
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

        let mut connection = self.storage.get_async_connection().await.unwrap();
        let _: RedisResult<bool> = connection
            .json_set(
                params.path.as_ref().to_string_lossy().to_string(),
                ".",
                &object,
            )
            .await;

        Ok(object)
    }

    async fn create_large_file<P: AsRef<Path> + Send + Sync>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst>> {
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
            params.user_id,
            ObjectVariant::LargeFile(LargeFile {
                tree: MerkleTree::build(blocks),
            }),
        );

        let mut connection = self.storage.get_async_connection().await.unwrap();
        let _: RedisResult<bool> = connection
            .json_set(
                params.path.as_ref().to_string_lossy().to_string(),
                ".",
                &object,
            )
            .await;

        Ok(object)
    }

    async fn get_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>> {
        let mut connection = self.storage.get_async_connection().await.unwrap();

        let object: RedisResult<String> = connection
            .json_get(path.as_ref().to_string_lossy().to_string(), ".")
            .await;

        match object {
            Ok(object) => {
                let object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();

                return match object.inner {
                    ObjectVariant::LargeFile(_) => Err(MetadataError::TryingToGetSmallButItLarge(
                        path.as_ref().to_string_lossy().to_string(),
                    )),
                    ObjectVariant::SmallFile(_) => Ok(object),
                };
            }
            Err(_) => Err(MetadataError::FileNotFoundError(
                path.as_ref().to_string_lossy().to_string(),
            )),
        }
    }

    async fn add_commit_to_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>> {
        let mut connection = self.storage.get_async_connection().await.unwrap();
        let object: String = connection
            .json_get(path.as_ref().to_string_lossy().to_string(), ".")
            .await
            .map_err(|_| {
                MetadataError::FileNotFoundError(path.as_ref().to_string_lossy().to_string())
            })?;

        let mut object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();
        return match object.inner {
            ObjectVariant::LargeFile(_) => Err(MetadataError::CannotAddBlockToLargeFileError(
                path.as_ref().to_string_lossy().to_string(),
            )),
            ObjectVariant::SmallFile(ref mut file) => {
                let response = self.data_node_client.create_blocks(1).await?;
                let block = &response.blocks[0];
                file.add_block(Block {
                    id: Uuid::from_slice(block.block_id.as_slice()).unwrap(/*Never panic*/),
                    part: block.part as usize,
                    dst: response.endpoint.clone(),
                    replicas: vec![],
                    checksum: 0u32,
                });

                let _: RedisResult<bool> = connection
                    .json_set(path.as_ref().to_string_lossy().to_string(), ".", &object)
                    .await;

                Ok(object)
            }
        };
    }

    async fn get_large_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>> {
        let mut connection = self.storage.get_async_connection().await.unwrap();
        let object: RedisResult<String> = connection
            .json_get(path.as_ref().to_string_lossy().to_string(), ".")
            .await;

        match object {
            Ok(object) => {
                let object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();
                return match object.inner {
                    ObjectVariant::LargeFile(_) => Ok(object),
                    ObjectVariant::SmallFile(_) => Err(MetadataError::TryingToGetLargeButItSmall(
                        path.as_ref().to_string_lossy().to_string(),
                    )),
                };
            }
            Err(_) => Err(MetadataError::FileNotFoundError(
                path.as_ref().to_string_lossy().to_string(),
            )),
        }
    }

    async fn delete_object<P: AsRef<Path> + Send + Sync>(
        &self,
        user_id: Uuid,
        path: P,
    ) -> MetadataResult<()> {
        let mut connection = self.storage.get_async_connection().await.unwrap();

        let object: RedisResult<String> = connection
            .json_get(path.as_ref().to_string_lossy().to_string(), ".")
            .await;

        return match object {
            Ok(object) => {
                let object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();

                if object.owner != user_id {
                    return Err(MetadataError::NoPermission(
                        path.as_ref().to_string_lossy().to_string(),
                    ));
                }

                let _: RedisResult<bool> = connection
                    .del(path.as_ref().to_string_lossy().to_string())
                    .await;

                let blocks = object.get_all_blocks();

                for block in blocks {
                    if let Err(err) = self
                        .data_node_client
                        .delete_block(block.id, block.part)
                        .await
                    {
                        tracing::error!("{}", err);
                    }
                }

                Ok(())
            }
            Err(_) => Err(MetadataError::FileNotFoundError(
                path.as_ref().to_string_lossy().to_string(),
            )),
        };
    }

    #[allow(clippy::single_match)]
    async fn add_checksum<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
        block_id: Uuid,
        part: usize,
        checksum: u32,
    ) {
        let mut connection = self.storage.get_async_connection().await.unwrap();
        let object: RedisResult<String> = connection
            .json_get(path.as_ref().to_string_lossy().to_string(), ".")
            .await;

        match object {
            Ok(object) => {
                let mut object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();
                object.update_block(block_id, part, checksum);

                let _: RedisResult<bool> = connection
                    .json_set(path.as_ref().to_string_lossy().to_string(), "$", &object)
                    .await;
            }
            Err(_) => (), /*TODO: Error Handle*/
        }
    }

    async fn get_files(&self, prefix: &str) -> Vec<Object<Self::Dst>> {
        let mut connection = self.storage.get_async_connection().await.unwrap();
        let keys: Vec<String> = connection.keys(format!("{}*", prefix)).await.unwrap();

        let mut files = vec![];

        for key in keys {
            let object: RedisResult<String> = connection.json_get(key, ".").await;

            match object {
                Ok(object) => {
                    let object: Object<Self::Dst> = serde_json::from_str(&object).unwrap();
                    println!("{:?}", object);
                    files.push(object);
                }
                Err(_) => todo!(), /*TODO: Handle error*/
            }
        }

        files
    }
}
