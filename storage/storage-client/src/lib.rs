use crate::client::StorageClient;
use crate::config::Config;
use uuid::Uuid;

pub mod client;
pub mod config;

#[tokio::test]
async fn create_small_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let file = tokio::fs::File::open("../../Cargo.lock").await.unwrap();
    client
        .create_small_file(Uuid::new_v4(), "test", file)
        .await
        .unwrap()
}

#[tokio::test]
async fn create_large_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let file = tokio::fs::File::open("../../CargoBig.lock").await.unwrap();
    client
        .create_large_file(Uuid::new_v4(), "test_big", file)
        .await
        .unwrap()
}

#[tokio::test]
async fn read_small_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let data = client
        .read_small_file_last_version(Uuid::new_v4(), "test")
        .await
        .unwrap();

    assert_eq!(54939, data.len());
}

#[tokio::test]
async fn read_large_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let data = client
        .read_large_file(Uuid::new_v4(), "test_big")
        .await
        .unwrap();

    assert_eq!(164820, data.len());
}

#[tokio::test]
async fn delete_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let _ = client.delete_file(Uuid::new_v4(), "test").await.unwrap();
}

#[tokio::test]
async fn add_commit_to_small_file_test() {
    let config = Config::try_from_file("../../ClientTest.toml").await;
    let client = StorageClient::new(config);
    let file = tokio::fs::File::open("../../Cargo.lock").await.unwrap();
    client
        .create_small_file(Uuid::new_v4(), "test", file)
        .await
        .unwrap();

    client
        .add_new_commit_to_small_file(
            Uuid::new_v4(),
            "test",
            &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        )
        .await
        .unwrap();

    let data = client
        .read_small_file_last_version(Uuid::new_v4(), "test")
        .await
        .unwrap();

    let _ = client.delete_file(Uuid::new_v4(), "test").await.unwrap();

    assert_eq!(16, data.len());
}
