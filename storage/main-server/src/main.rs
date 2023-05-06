use crate::config::Config;
use crate::service::metadata_controller::MetadataController;
use std::net::SocketAddr;
use tonic::transport::Server;
use uuid::Uuid;
use crate::storage_types::commit_types::block::Block;
use crate::storage_types::commit_types::merkle_tree::MerkleTree;

mod config;
mod constants;
mod data_node_client;
mod service;
mod storage_types;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let blocks = vec![
        Block {
            id: Uuid::new_v4(),
            part: 0,
            dst: "".to_string(),
            replicas: vec![],
            checksum: 128,
        },
        Block {
            id: Uuid::new_v4(),
            part: 1,
            dst: "".to_string(),
            replicas: vec![],
            checksum: 129,
        },
        Block {
            id: Uuid::new_v4(),
            part: 2,
            dst: "".to_string(),
            replicas: vec![],
            checksum: 131,
        },
        Block {
            id: Uuid::new_v4(),
            part: 3,
            dst: "".to_string(),
            replicas: vec![],
            checksum: 132,
        },
        Block {
            id: Uuid::new_v4(),
            part: 4,
            dst: "".to_string(),
            replicas: vec![],
            checksum: 133,
        }
    ];

    let tree = MerkleTree::build(blocks);
    println!("{}", tree.layers());

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
