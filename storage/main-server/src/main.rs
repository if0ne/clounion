use crate::config::Config;
use crate::service::metadata_controller::MetadataController;
use crate::storage_types::commit_types::block::Block;
use crate::storage_types::commit_types::merkle_tree::MerkleTree;
use std::net::SocketAddr;
use tonic::transport::Server;
use uuid::Uuid;

mod config;
mod constants;
mod data_node_client;
mod service;
mod storage_types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::try_from_file("MainServerTest.toml").await;
    let addr = format!("{}:{}", config.self_address, config.port)
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");

    let (_, health_service) = tonic_health::server::health_reporter();
    let metadata_service = MetadataController::new().await;

    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(metadata_service)
        .serve(addr)
        .await?;

    Ok(())
}
