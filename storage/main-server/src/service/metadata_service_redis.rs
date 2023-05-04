use super::metadata_service::{CreationParam, MetadataResult, MetadataService};
use crate::storage_types::object::Object;
use std::path::Path;
use async_trait::async_trait;

pub struct MetaServiceRedis {
    storage: redis::Client,
}

#[async_trait]
impl MetadataService for MetaServiceRedis {
    type Dst = String;
    type Hash = u32;

    async fn create_small_file<P: AsRef<Path> + Send>(
        &self,
        params: CreationParam<P>,
    ) -> MetadataResult<Object<Self::Dst, Self::Hash>> {
        todo!()
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
