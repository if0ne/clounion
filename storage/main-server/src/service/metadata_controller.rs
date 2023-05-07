mod proto_main_server {
    tonic::include_proto!("main_server");
}

use crate::service::metadata_controller::proto_main_server::main_server_service_server::MainServerServiceServer;
use crate::service::metadata_controller::proto_main_server::AddChecksumRequest;
use crate::service::metadata_service::{CreationParam, MetadataService};
use crate::service::metadata_service_redis::MetaServiceRedis;
use crate::storage_types::object::ObjectVariant;
use proto_main_server::main_server_service_server::MainServerService;
use proto_main_server::{
    AddCommitSmallFileRequest, BlockInfo, CreateFileRequest, CreateLargeFileResponse,
    CreateSmallFileResponse, DeleteFileRequest, EmptyResponse, GetLargeFileRequest,
    GetSmallFileLastVersionRequest, GetSmallFileRequest, LargeFileResponse,
};
use shared::main_server_error::MetadataError;
use smallvec::SmallVec;
use tonic::{Request, Response, Status};
use uuid::Uuid;

pub struct MetadataController {
    metadata_service: MetaServiceRedis,
}

impl MetadataController {
    pub async fn new(service: MetaServiceRedis) -> MainServerServiceServer<Self> {
        MainServerServiceServer::new(Self {
            metadata_service: service,
        })
    }
}

#[tonic::async_trait]
impl MainServerService for MetadataController {
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
        todo!()
    }

    async fn get_small_file(
        &self,
        request: Request<GetSmallFileRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        todo!()
    }

    async fn get_last_version_small_file(
        &self,
        request: Request<GetSmallFileLastVersionRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        todo!()
    }

    async fn add_commit_to_small_file(
        &self,
        request: Request<AddCommitSmallFileRequest>,
    ) -> Result<Response<BlockInfo>, Status> {
        todo!()
    }

    async fn get_large_file(
        &self,
        request: Request<GetLargeFileRequest>,
    ) -> Result<Response<LargeFileResponse>, Status> {
        todo!()
    }

    async fn delete_file(
        &self,
        request: Request<DeleteFileRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        todo!()
    }

    async fn add_checksum(
        &self,
        request: Request<AddChecksumRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        todo!()
    }
}
