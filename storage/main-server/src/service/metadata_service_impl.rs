use super::metadata_service::{CreationParam, MetadataResult, MetadataService};
use crate::storage_types::commit_types::block::Block;
use crate::storage_types::commit_types::commit::Commits;
use crate::storage_types::commit_types::sequence::Sequence;
use crate::storage_types::object::{Object, ObjectVariant};
use crate::storage_types::small_file::SmallFile;
use async_trait::async_trait;
use fast_str::FastStr;
use smallvec::SmallVec;
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct MetadataServiceImpl {
    database: RwLock<HashMap<String, Object<String, u32>>>,
}

#[async_trait]
impl MetadataService for MetadataServiceImpl {
    type Dst = String;
    type Hash = u32;

    async fn create_small_file<P: AsRef<Path>>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        let mut db = self.database.write().await;
        let mut obj = Object::new(
            FastStr::from_string(params.path.as_ref().to_string_lossy().to_string()),
            ObjectVariant::SmallFile(SmallFile {
                commits: Commits::Sequence(Sequence {
                    seq: vec![Block {
                        id: Uuid::from_u128(1),
                        dst: "Test/1".to_string(),
                        replicas: SmallVec::new(),
                        checksum: 1,
                    }],
                }),
            }),
        );
        db.insert(
            params.path.as_ref().to_string_lossy().to_string(),
            obj.clone(),
        );

        Ok(obj)
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
