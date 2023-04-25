use crate::constants::MAX_GROUP_ACCESS;
use crate::storage_types::object::Object;
use async_trait::async_trait;
use smallvec::SmallVec;
use std::path::Path;
use uuid::Uuid;

pub type MetadataResult<T> = Result<T, shared::main_server_error::MetadataError>;

#[async_trait]
pub trait MetadataService {
    type Dst;
    type Hash;

    async fn create_small_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn create_large_file<P: AsRef<Path> + Send>(
        &self,
        path: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn get_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn get_large_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn add_commit_to_small_file<P: AsRef<Path> + Send>(
        &self,
        path: P,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>>;

    async fn delete_object<P: AsRef<Path> + Send>(&self, path: P) -> MetadataResult<()>;
}

pub struct CreationParam<P: AsRef<Path>> {
    pub user_id: Uuid,
    pub group_id: SmallVec<[Uuid; MAX_GROUP_ACCESS]>,
    pub path: P,
    pub size: u128,
}

unsafe impl<P: AsRef<Path>> Send for CreationParam<P> {}
