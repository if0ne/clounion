use crate::constants::MAX_GROUP_ACCESS;
use crate::storage_types::object::Object;
use async_trait::async_trait;
use serde::Serialize;
use smallvec::SmallVec;
use std::fmt::Debug;
use std::path::Path;
use uuid::Uuid;

pub type MetadataResult<T> = Result<T, shared::main_server_error::MetadataError>;

#[async_trait]
pub trait MetadataService: Send + Sync + Sync {
    type Dst: Serialize + Debug;

    async fn create_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst>>;

    async fn create_large_file<P: AsRef<Path> + Send + Sync>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst>>;

    async fn get_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>>;

    async fn add_commit_to_small_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>>;

    async fn get_large_file<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst>>;

    async fn delete_object<P: AsRef<Path> + Send + Sync>(&self, path: P) -> MetadataResult<()>;

    async fn add_checksum<P: AsRef<Path> + Send + Sync>(
        &self,
        path: P,
        block_id: Uuid,
        part: usize,
        checksum: u32,
    );
    async fn get_files(&self, prefix: &str) -> Vec<Object<Self::Dst>>;
}

#[derive(Debug)]
pub struct CreationParam<P: AsRef<Path>> {
    pub user_id: Uuid,
    pub group_id: SmallVec<[Uuid; MAX_GROUP_ACCESS]>,
    pub path: P,
    pub size: usize,
}

unsafe impl<P: AsRef<Path>> Send for CreationParam<P> {}
unsafe impl<P: AsRef<Path>> Sync for CreationParam<P> {}
