use crate::constants::MAX_GROUP_ACCESS;
use crate::service::error::MetadataError;
use crate::storage_types::object::Object;
use async_trait::async_trait;
use smallvec::SmallVec;
use std::path::Path;
use uuid::Uuid;

pub type MetadataResult<T> = Result<T, MetadataError>;

#[async_trait]
pub trait MetadataService<T, Hash> {
    async fn create_small_file<P: AsRef<Path>>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<T, Hash>>;

    async fn create_large_file<P: AsRef<Path>>(
        &self,
        path: CreationParam<P>,
    ) -> MetadataResult<Object<T, Hash>>;
}

pub struct CreationParam<P: AsRef<Path>> {
    user_id: Uuid,
    group_id: SmallVec<[Uuid; MAX_GROUP_ACCESS]>,
    path: P,
}
