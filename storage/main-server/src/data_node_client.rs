mod proto_data_node {
    tonic::include_proto!("data_node");
}

mod proto_registry {
    tonic::include_proto!("registry_main_server");
}

use proto_registry::{RegistryRequest, RegistryResponse};
use proto_registry::registry_data_node_service_server::RegistryDataNodeService;
use proto_data_node::{
    data_node_service_client::DataNodeServiceClient,
};
use tokio::sync::RwLock;
use tonic::transport::Channel;
use tonic::{Request, Response, Status};

pub struct DataNodeClient {
    inner: RwLock<Option<DataNodeServiceClient<Channel>>>,
    data_nodes: RwLock<Vec<String>>,
}

impl DataNodeClient {
    pub fn new() -> Self {
        Self {
            inner: RwLock::new(None),
            data_nodes: RwLock::new(vec![]),
        }
    }
}

#[tonic::async_trait]
impl RegistryDataNodeService for DataNodeClient {
    async fn registry(
        &self,
        request: Request<RegistryRequest>,
    ) -> Result<Response<RegistryResponse>, Status> {
        let request = request.into_inner();

        let mut writer = self.data_nodes.write().await;
        writer.push(request.data_node_address);

        let endpoints = writer.iter().map(|a| Channel::from_shared(a.as_bytes()).unwrap());
        let channel = Channel::balance_list(endpoints);

        let mut writer = self.inner.write().await;
        *writer = Some(DataNodeServiceClient::new(channel));

        Ok(Response::new(RegistryResponse {}))
    }
}
