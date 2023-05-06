use crate::constants::MAX_GROUP_ACCESS;
use crate::storage_types::object::Object;
use async_trait::async_trait;
use serde::Serialize;
use smallvec::SmallVec;
use std::path::Path;
use uuid::Uuid;
use zerocopy::AsBytes;

pub type MetadataResult<T> = Result<T, shared::main_server_error::MetadataError>;

#[async_trait]
pub trait MetadataService {
    type Dst: Serialize;
    type Hash: Serialize + Copy + AsBytes;

    async fn create_small_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn create_large_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn get_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn get_small_file_last_version<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn add_commit_to_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn get_large_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn delete_object<P: AsRef<Path> + Send>(&self, path: P) -> MetadataResult<()>;
}

#[derive(Debug)]
pub struct CreationParam<P: AsRef<Path>> {
    pub user_id: Uuid,
    pub group_id: SmallVec<[Uuid; MAX_GROUP_ACCESS]>,
    pub path: P,
    pub size: usize,
}

unsafe impl<P: AsRef<Path>> Send for CreationParam<P> {}
