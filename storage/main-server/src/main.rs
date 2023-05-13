use crate::config::Config;
use crate::data_node_client::DataNodeClient;
use crate::service::metadata_controller::MetadataController;
use crate::service::metadata_service_redis::MetaServiceRedis;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;

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

    let data_node_client = Arc::new(DataNodeClient::new().await);
    let metadata_service_redis = {
        let redis = redis::Client::open(config.database_connection.clone()).unwrap();
        MetaServiceRedis::new(redis, data_node_client.clone(), config).await
    };
    let (metadata_service, metadata_service_api) =
        MetadataController::new(Arc::new(metadata_service_redis)).await;

    tracing::info!("Starting server on {}:{}", addr.ip(), addr.port());
    Server::builder()
        .accept_http1(true)
        .add_service(health_service)
        .add_service(metadata_service)
        .add_service(metadata_service_api)
        .add_service(data_node_client.get_service())
        .serve(addr)
        .await?;

    Ok(())
}
