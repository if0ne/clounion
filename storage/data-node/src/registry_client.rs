mod proto_registry_main_server {
    tonic::include_proto!("registry_main_server");
}

use tonic::transport::{Channel, Endpoint};

use proto_registry_main_server::{
    registry_data_node_service_client::RegistryDataNodeServiceClient, RegistryRequest,
};

use super::config::Config;

pub struct RegistryClient {
    inner: RegistryDataNodeServiceClient<Channel>,
}

impl RegistryClient {
    pub async fn new(main_server_addr: &str) -> Result<Self, tonic::transport::Error> {
        let endpoint = Endpoint::try_from(format!("http://{}", main_server_addr))?;
        let channel = endpoint.connect().await?;

        Ok(Self {
            inner: RegistryDataNodeServiceClient::new(channel),
        })
    }

    pub async fn send_registry(&mut self, config: &Config) -> Result<(), ()> {
        let response = self
            .inner
            .registry(RegistryRequest {
                data_node_address: config.self_address.clone(),
                block_size: config.block_size as u64,
            })
            .await;

        if let Err(err) = response {
            /*match err.try_into() {
                Ok(err) => return Err(err),
                Err(err) => tracing::error!("{}", err.to_string()),
            }*/
        }

        Ok(())
    }
}
