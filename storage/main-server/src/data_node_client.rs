mod proto_data_node {
    tonic::include_proto!("data_node");
}

mod proto_registry {
    tonic::include_proto!("registry_main_server");
}

use crate::data_node_client::proto_data_node::{BlockInfo, CreateBlocksRequest, CreateBlocksResponse, DeleteBlockRequest, EmptyResponse};
use crate::data_node_client::proto_registry::registry_data_node_service_server::RegistryDataNodeServiceServer;
use proto_data_node::data_node_service_client::DataNodeServiceClient;
use proto_registry::registry_data_node_service_server::RegistryDataNodeService;
use proto_registry::{RegistryRequest, RegistryResponse};
use shared::data_node_error::DataNodeError;
use shared::main_server_error::MetadataError;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Response, Status};
use uuid::Uuid;
use crate::storage_types::commit_types::block::Block;

pub struct DataNodeClient {
    inner: RwLock<Option<DataNodeServiceClient<Channel>>>,
    data_nodes: RwLock<Vec<String>>,
}

impl DataNodeClient {
    pub async fn new() -> Self {
        Self {
            inner: RwLock::new(None),
            data_nodes: RwLock::new(vec![]),
        }
    }

    pub fn get_service(self: Arc<Self>) -> RegistryDataNodeServiceServer<Self> {
        RegistryDataNodeServiceServer::from_arc(self.clone())
    }

    pub async fn create_blocks(&self, count: usize) -> Result<CreateBlocksResponse, MetadataError> {
        if let Some(ref mut client) = *self.inner.write().await {
            return match client
                .create_blocks(CreateBlocksRequest {
                    count: count as u64,
                })
                .await
            {
                Ok(block) => Ok(block.into_inner()),
                Err(error) => Err(MetadataError::CreateBlocksResponseError(format!(
                    "Error from data node while creating blocks"
                ))),
            };
        }

        Err(MetadataError::CreateFileError(format!(
            "No one of data nodes are connected"
        )))
    }

    pub async fn delete_block(&self, block_id: Uuid, part: usize) -> Result<(), MetadataError>  {
        if let Some(ref mut client) = *self.inner.write().await {
            match client
                .delete_block(DeleteBlockRequest {
                    block: Some(BlockInfo {
                        block_id: block_id.as_bytes().to_vec(),
                        part: part as u64,
                    }),
                })
                .await {
                Ok(_) => {}
                Err(info) => {
                    tracing::error!("Error to delete block: {:?}", info)
                }
            }
        }

        Err(MetadataError::CreateFileError(format!(
            "No one of data nodes are connected"
        )))
    }
}

#[tonic::async_trait]
impl RegistryDataNodeService for DataNodeClient {
    async fn registry(
        &self,
        request: Request<RegistryRequest>,
    ) -> Result<Response<RegistryResponse>, Status> {
        /// TODO: Переделать
        let request = request.into_inner();
        tracing::info!("Connecting {}", request.data_node_address);

        let endpoint = Endpoint::try_from(format!("http://{}", request.data_node_address)).unwrap();
        let channel = endpoint.connect().await.unwrap();
        let client = DataNodeServiceClient::new(channel);

        *self.inner.write().await = Some(client);

        tracing::info!("Connected {}", request.data_node_address);

        /*let request = request.into_inner();

        let mut writer = self.data_nodes.write().await;
        writer.push(request.data_node_address);

        let endpoints = writer
            .iter()
            .map(|a| Channel::from_shared(a.as_bytes()).unwrap());
        let channel = Channel::balance_list(endpoints);

        let mut writer = self.inner.write().await;
        *writer = Some(DataNodeServiceClient::new(channel));*/

        Ok(Response::new(RegistryResponse {}))
    }
}
