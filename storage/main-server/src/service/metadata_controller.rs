mod proto_main_server {
    tonic::include_proto!("main_server");
}

use crate::service::metadata_controller::proto_main_server::AddChecksumRequest;
use proto_main_server::main_server_service_server::MainServerService;
use proto_main_server::{
    AddCommitSmallFileRequest, BlockInfo, CreateFileRequest, CreateLargeFileResponse,
    CreateSmallFileResponse, DeleteFileRequest, EmptyResponse, GetLargeFileRequest,
    GetSmallFileLastVersionRequest, GetSmallFileRequest, LargeFileResponse,
};
use tonic::{Request, Response, Status};
use crate::service::metadata_controller::proto_main_server::main_server_service_server::MainServerServiceServer;

pub struct MetadataController {}

impl MetadataController {
    pub async fn new() -> MainServerServiceServer<Self> {
        MainServerServiceServer::new(Self {

        })
    }
}

#[tonic::async_trait]
impl MainServerService for MetadataController {
    async fn create_small_file(
        &self,
        request: Request<CreateFileRequest>,
    ) -> Result<Response<CreateSmallFileResponse>, Status> {
        todo!()
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
