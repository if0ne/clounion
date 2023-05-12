pub mod proto_main_server {
    tonic::include_proto!("main_server");
}

use crate::main_server_client::proto_main_server::{AddChecksumRequest, BlockInfo};
use proto_main_server::main_server_service_client::MainServerServiceClient;
use tokio::sync::RwLock;
use tonic::transport::{Channel, Endpoint};

pub struct MainServerClient {
    inner: RwLock<MainServerServiceClient<Channel>>,
}

impl MainServerClient {
    pub async fn new(main_server_addr: &str) -> Self {
        //TODO: Error handle
        let endpoint = Endpoint::try_from(format!("http://{}", main_server_addr)).unwrap();
        let channel = endpoint.connect().await.unwrap();

        let client = MainServerServiceClient::new(channel);

        Self {
            inner: RwLock::new(client),
        }
    }

    pub async fn add_checksum(&self, filename: &str, block: BlockInfo, checksum: u32) {
        // TODO: Error handle
        let status = self
            .inner
            .write()
            .await
            .add_checksum(AddChecksumRequest {
                filename: filename.to_string(),
                block: Some(block),
                checksum,
            })
            .await;

        if status.is_err() {
            tracing::error!("Error to send checksum for {}", filename);
        }
    }
}
