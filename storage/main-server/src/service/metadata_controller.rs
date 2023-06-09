mod proto_main_server {
    tonic::include_proto!("main_server");
}

mod proto_main_server_api {
    tonic::include_proto!("main_server_api");
}

use crate::service::metadata_service::{CreationParam, MetadataService};
use crate::service::metadata_service_redis::MetaServiceRedis;
use crate::storage_types::object::ObjectVariant;
use proto_main_server::main_server_service_server::MainServerService;
use proto_main_server::main_server_service_server::MainServerServiceServer;
use proto_main_server::AddChecksumRequest;
use proto_main_server::EmptyResponse as EmptyResponseInternal;
use proto_main_server_api::main_server_service_api_server::{
    MainServerServiceApi, MainServerServiceApiServer,
};
use proto_main_server_api::{
    AddCommitSmallFileRequest, BlockInfo, CreateFileRequest, CreateLargeFileResponse,
    CreateSmallFileResponse, DeleteFileRequest, EmptyResponse, FileRequest, FileResponse,
    GetLargeFileRequest, GetSmallFileLastVersionRequest, GetSmallFileRequest, LargeFileResponse,
    ObjectResponse,
};
use shared::main_server_error::MetadataError;
use smallvec::SmallVec;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;
use zerocopy::AsBytes;

pub struct MetadataController {
    metadata_service: Arc<MetaServiceRedis>,
}

impl MetadataController {
    pub async fn new(
        service: Arc<MetaServiceRedis>,
    ) -> (
        MainServerServiceServer<Self>,
        MainServerServiceApiServer<Self>,
    ) {
        (
            MainServerServiceServer::new(Self {
                metadata_service: service.clone(),
            }),
            MainServerServiceApiServer::new(Self {
                metadata_service: service,
            }),
        )
    }
}

#[tonic::async_trait]
impl MainServerServiceApi for MetadataController {
    async fn create_small_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> Result<Response<CreateSmallFileResponse>, Status> {
        let request = request.into_inner();
        let user_id = Uuid::from_slice(&request.user_id)
            .map_err(|_| MetadataError::WrongUuid(format!("{:?}", &request.user_id)))?;

        let file = self
            .metadata_service
            .create_small_file(CreationParam {
                user_id,
                group_id: SmallVec::new(),
                path: request.filename,
                size: request.size as usize,
            })
            .await?;

        if let ObjectVariant::SmallFile(file) = file.inner {
            let block = file.commits.last();
            Ok(Response::new(CreateSmallFileResponse {
                block: Some(BlockInfo {
                    block_id: block.id.as_bytes().to_vec(),
                    part: block.part as u64,
                    endpoint: block.dst.clone(),
                }),
            }))
        } else {
            unreachable!()
        }
    }

    async fn create_large_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> Result<Response<CreateLargeFileResponse>, Status> {
        let request = request.into_inner();
        let user_id = Uuid::from_slice(&request.user_id)
            .map_err(|_| MetadataError::WrongUuid(format!("{:?}", &request.user_id)))?;

        let file = self
            .metadata_service
            .create_large_file(CreationParam {
                user_id,
                group_id: SmallVec::new(),
                path: request.filename,
                size: request.size as usize,
            })
            .await?;

        if let ObjectVariant::LargeFile(file) = file.inner {
            let blocks = file
                .tree
                .leaves()
                .iter()
                .map(|el| BlockInfo {
                    block_id: el.id.as_bytes().to_vec(),
                    part: el.part as u64,
                    endpoint: el.dst.clone(),
                })
                .collect();

            Ok(Response::new(CreateLargeFileResponse { blocks }))
        } else {
            unreachable!()
        }
    }

    async fn get_small_file(
        &self,
        request: Request<GetSmallFileRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        let request = request.into_inner();

        let file = self
            .metadata_service
            .get_small_file(request.filename.clone())
            .await?;

        if file.owner.as_bytes() != request.user_id.as_bytes() {
            return Err(MetadataError::NoPermission(request.filename).into());
        }

        if let ObjectVariant::SmallFile(file) = file.inner {
            let block = file.commits.index(request.index as usize);

            if let Some(block) = block {
                Ok(Response::new(BlockInfo {
                    block_id: block.id.as_bytes().to_vec(),
                    part: block.part as u64,
                    endpoint: block.dst.clone(),
                }))
            } else {
                Err(MetadataError::WrongSmallFileVersion(request.filename).into())
            }
        } else {
            unreachable!()
        }
    }

    async fn get_last_version_small_file(
        &self,
        request: Request<GetSmallFileLastVersionRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        let request = request.into_inner();

        let file = self
            .metadata_service
            .get_small_file(request.filename.clone())
            .await?;

        if file.owner.as_bytes() != request.user_id.as_bytes() {
            return Err(MetadataError::NoPermission(request.filename).into());
        }

        if let ObjectVariant::SmallFile(file) = file.inner {
            let block = file.commits.last();
            Ok(Response::new(BlockInfo {
                block_id: block.id.as_bytes().to_vec(),
                part: block.part as u64,
                endpoint: block.dst.clone(),
            }))
        } else {
            unreachable!()
        }
    }

    async fn add_commit_to_small_file(
        &self,
        request: Request<AddCommitSmallFileRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        let request = request.into_inner();

        let file = self
            .metadata_service
            .add_commit_to_small_file(request.filename.clone())
            .await?;

        if file.owner.as_bytes() != request.user_id.as_bytes() {
            return Err(MetadataError::NoPermission(request.filename).into());
        }

        if let ObjectVariant::SmallFile(file) = file.inner {
            let block = file.commits.last();
            Ok(Response::new(BlockInfo {
                block_id: block.id.as_bytes().to_vec(),
                part: block.part as u64,
                endpoint: block.dst.clone(),
            }))
        } else {
            unreachable!()
        }
    }

    async fn get_large_file(
        &self,
        request: Request<GetLargeFileRequest>,
    ) -> Result<Response<LargeFileResponse>, Status> {
        let request = request.into_inner();

        let file = self
            .metadata_service
            .get_large_file(request.filename.clone())
            .await?;

        if file.owner.as_bytes() != request.user_id.as_bytes() {
            return Err(MetadataError::NoPermission(request.filename).into());
        }

        if let ObjectVariant::LargeFile(file) = file.inner {
            let blocks = file
                .tree
                .leaves()
                .iter()
                .map(|el| BlockInfo {
                    block_id: el.id.as_bytes().to_vec(),
                    part: el.part as u64,
                    endpoint: el.dst.clone(),
                })
                .collect();

            Ok(Response::new(LargeFileResponse { blocks }))
        } else {
            unreachable!()
        }
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let request = request.into_inner();

        let user_id = Uuid::from_slice(&request.user_id)
            .map_err(|_| MetadataError::WrongUuid(format!("{:?}", &request.user_id)))?;

        self.metadata_service
            .delete_object(user_id, request.filename)
            .await?;

        Ok(Response::new(EmptyResponse {}))
    }

    async fn get_files(
        &self,
        request: Request<FileRequest>,
    ) -> Result<Response<FileResponse>, Status> {
        let request = request.into_inner();
        let objects = self.metadata_service.get_files(&request.prefix).await;

        let objects = objects
            .into_iter()
            .filter(|el| el.owner.as_bytes() == request.user_id.as_bytes())
            .map(|el| ObjectResponse {
                filename: el.name.to_string(),
                r#type: match el.inner {
                    ObjectVariant::SmallFile(_) => 0,
                    ObjectVariant::LargeFile(_) => 1,
                },
            })
            .collect();

        Ok(Response::new(FileResponse { files: objects }))
    }
}

#[tonic::async_trait]
impl MainServerService for MetadataController {
    async fn add_checksum(
        &self,
        request: Request<AddChecksumRequest>,
    ) -> Result<Response<EmptyResponseInternal>, Status> {
        let request = request.into_inner();
        let block = request.block.unwrap();

        self.metadata_service
            .add_checksum(
                request.filename,
                Uuid::from_slice(&block.block_id).unwrap(),
                block.part as usize,
                request.checksum,
            )
            .await;

        Ok(Response::new(EmptyResponseInternal {}))
    }
}
