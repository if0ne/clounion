use crate::config::Config;
use crate::data_node_controller::DataNodeController;
use crate::data_node_info::DataNodeInfo;
use crate::main_server_client::MainServerClient;
use crate::registry_client::RegistryClient;
use std::net::SocketAddr;
use std::thread;
use tonic::transport::Server;

mod block_storage;
mod block_storage_service;
mod config;
mod data_node_controller;
mod data_node_info;
mod disk_stats;
mod main_server_client;
mod registry_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    sysinfo::set_open_files_limit(0);
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::try_from_file("DataNodeTest.toml").await;

    let main_server_client = MainServerClient::new(config.get_main_server_addr()).await;
    let data_node_info = DataNodeInfo::new(config.clone()).await;
    let addr = format!("{}:{}", data_node_info.self_address, data_node_info.port)
        .parse::<SocketAddr>()
        .expect("Unable to parse socket address");

    let (_, health_service) = tonic_health::server::health_reporter();
    let data_node = DataNodeController::get_service(data_node_info, main_server_client)
        .await
        .unwrap();

    tracing::info!("Starting server on {}:{}", addr.ip(), addr.port());

    tokio::spawn(async move {
        Server::builder()
            .accept_http1(true)
            .add_service(health_service)
            .add_service(data_node)
            .serve(addr)
            .await
    });

    thread::sleep(std::time::Duration::new(10, 0));
    let registry_client = RegistryClient::new(config.get_main_server_addr()).await;
    match registry_client {
        Ok(mut client) => {
            let response = client.send_registry(&config).await;
            if let Err(response) = response {
                tracing::error!("{}", response.to_string());
            }
        }
        Err(err) => {
            tracing::error!(
                "{}. Please start up the main server and restart data node.",
                err.to_string()
            )
        }
    };

    tokio::signal::ctrl_c().await?;
    Ok(())
}
