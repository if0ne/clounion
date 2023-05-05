mod proto_data_node {
    tonic::include_proto!("data_node");
}

mod proto_registry {
    tonic::include_proto!("registry_main_server");
}

use crate::data_node_client::proto_data_node::{CreateBlocksRequest, CreateBlocksResponse};
use proto_data_node::data_node_service_client::DataNodeServiceClient;
use proto_registry::registry_data_node_service_server::RegistryDataNodeService;
use proto_registry::{RegistryRequest, RegistryResponse};
use shared::data_node_error::DataNodeError;
use shared::main_server_error::MetadataError;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};
use tonic::{Request, Response, Status};

pub struct DataNodeClient {
    inner: RwLock<Option<DataNodeServiceClient<Channel>>>,
    data_nodes: RwLock<Vec<String>>,
}

impl DataNodeClient {
    pub async fn new() -> Self {
        /// TODO: Переделать
        let client = DataNodeServiceClient::connect("http://[::1]:40000")
            .await
            .unwrap();
        Self {
            inner: RwLock::new(Some(client)),
            data_nodes: RwLock::new(vec![]),
        }
    }

    pub async fn create_blocks(&self, count: usize) -> Result<CreateBlocksResponse, MetadataError> {
        if let Some(ref mut client) = *self.inner.write().await {
            match client
                .create_blocks(CreateBlocksRequest {
                    count: count as u64,
                })
                .await
            {
                Ok(block) => {
                    return Ok(block.into_inner());
                }
                Err(error) => {
                    return Err(MetadataError::CreateBlocksResponseError(format!(
                        "Error from data node while creating blocks"
                    )));
                }
            }
        }

        Err(MetadataError::CreateSmallFileError(format!(
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
