pub mod proto_data_node {
    tonic::include_proto!("data_node");
}

use crate::data_node_controller::proto_data_node::{
    data_node_service_server::{DataNodeService, DataNodeServiceServer},
    CreateBlocksRequest, CreateBlocksResponse, DeleteBlocksRequest, EmptyResponse,
    ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest,
};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};
use crate::data_node_info::DataNodeInfo;

#[derive(Clone)]
pub struct DataNodeController {
}

impl DataNodeController {
    pub async fn get_service(data_node_info: DataNodeInfo) -> DataNodeServiceServer<Self> {
        DataNodeServiceServer::new(Self {
        })
    }
}

#[tonic::async_trait]
impl DataNodeService for DataNodeController {
    type ReadBlockStream = ReceiverStream<Result<ReadBlockResponse, Status>>;

    async fn create_blocks(
        &self,
        request: Request<CreateBlocksRequest>,
    ) -> Result<Response<CreateBlocksResponse>, tonic::Status> {
        todo!()
    }

    async fn read_block(
        &self,
        request: Request<ReadBlockRequest>,
    ) -> Result<Response<Self::ReadBlockStream>, Status> {
        todo!()
    }

    async fn update_block(
        &self,
        request: Request<UpdateBlockRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        todo!()
    }

    async fn delete_block(
        &self,
        request: Request<DeleteBlocksRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        todo!()
    }
}
