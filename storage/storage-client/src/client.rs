use crate::client::proto_data_node_api::data_node_service_api_client::DataNodeServiceApiClient;
use crate::client::proto_data_node_api::{Range, ReadBlockRequest, UpdateBlockRequest};
use crate::client::proto_main_server_api::main_server_service_api_client::MainServerServiceApiClient;
use crate::client::proto_main_server_api::{
    AddCommitSmallFileRequest, CreateFileRequest, DeleteFileRequest, GetLargeFileRequest,
    GetSmallFileLastVersionRequest,
};
use crate::config::Config;
use futures::StreamExt;
use tokio::io::AsyncReadExt;
use uuid::Uuid;

mod proto_data_node_api {
    tonic::include_proto!("data_node_api");
}

mod proto_main_server_api {
    tonic::include_proto!("main_server_api");
}

pub struct StorageClient {
    config: Config,
}

impl StorageClient {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub async fn create_small_file(
        &self,
        user_id: Uuid,
        filename: &str,
        mut file: tokio::fs::File,
    ) -> Result<(), StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let file_size = file.metadata().await.unwrap().len();

        let remote_file = main_server_client
            .create_small_file(CreateFileRequest {
                filename: filename.to_string(),
                user_id: user_id.to_bytes_le().to_vec(),
                group_ids: vec![],
                size: file_size,
            })
            .await
            .map_err(|e| StorageClientError::CreateFileError)?
            .into_inner();

        let block = remote_file.block.unwrap();

        let mut data_node_client =
            DataNodeServiceApiClient::connect(format!("http://{}", block.endpoint))
                .await
                .map_err(|_| StorageClientError::WrongDatanodeAddressError)?;

        //TODO: Error handle
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).await.unwrap();

        let update_info = UpdateBlockRequest {
            filename: filename.to_string().clone(),
            block_id: block.block_id.clone(),
            part: block.part,
            range: Some(Range {
                start: 0,
                end: buffer.len() as u64,
            }),
            data: buffer,
            hash: 0,
        };

        let stream = tokio_stream::iter(std::iter::once(update_info));

        let _ = data_node_client
            .update_block(stream)
            .await
            .map_err(|_| StorageClientError::UpdateBlockError)?
            .into_inner();

        Ok(())
    }

    pub async fn create_large_file(
        &self,
        user_id: Uuid,
        filename: &str,
        mut file: tokio::fs::File,
    ) -> Result<(), StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let file_size = file.metadata().await.unwrap().len();

        let remote_file = main_server_client
            .create_large_file(CreateFileRequest {
                filename: filename.to_string(),
                user_id: user_id.to_bytes_le().to_vec(),
                group_ids: vec![],
                size: file_size,
            })
            .await
            .map_err(|e| StorageClientError::CreateFileError)?
            .into_inner();

        //TODO: Error handle
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).await.unwrap();

        let chunks = buffer.chunks(self.config.block_size);

        assert_eq!(chunks.len(), remote_file.blocks.len());

        let chunks = chunks
            .zip(remote_file.blocks)
            .map(move |(data, block)| {
                (
                    UpdateBlockRequest {
                        filename: filename.to_string().clone(),
                        block_id: block.block_id.clone(),
                        part: block.part,
                        data: data.to_vec(),
                        range: Some(Range {
                            start: 0,
                            end: data.len() as u64,
                        }),
                        hash: 0,
                    },
                    block.endpoint,
                )
            })
            .collect::<Vec<(UpdateBlockRequest, String)>>();

        //TODO: Join all
        for (block, endpoint) in chunks {
            let mut data_node_client =
                DataNodeServiceApiClient::connect(format!("http://{}", endpoint))
                    .await
                    .map_err(|_| StorageClientError::WrongDatanodeAddressError)?;

            let stream = tokio_stream::iter(std::iter::once(block));

            let _ = data_node_client
                .update_block(stream)
                .await
                .map_err(|_| StorageClientError::UpdateBlockError)?
                .into_inner();
        }

        Ok(())
    }

    pub async fn read_small_file_last_version(
        &self,
        user_id: Uuid,
        filename: &str,
    ) -> Result<Vec<u8>, StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let remote_file = main_server_client
            .get_last_version_small_file(GetSmallFileLastVersionRequest {
                filename: filename.to_string(),
                user_id: user_id.as_bytes().to_vec(),
                group_ids: vec![],
            })
            .await
            .map_err(|e| StorageClientError::ReadSmallFileError)?
            .into_inner();

        let mut data_node_client =
            DataNodeServiceApiClient::connect(format!("http://{}", remote_file.endpoint))
                .await
                .map_err(|_| StorageClientError::WrongDatanodeAddressError)?;

        let mut data = vec![];

        let response = data_node_client
            .read_block(ReadBlockRequest {
                part: remote_file.part,
                block_id: remote_file.block_id,
            })
            .await;

        return if let Ok(stream) = response {
            let mut stream = stream.into_inner();
            while let Some(part) = stream.next().await {
                if let Ok(part) = part {
                    data.push(part.data);
                } else {
                    return Err(StorageClientError::ReadSmallFileError);
                }
            }

            Ok(data.into_iter().flatten().collect())
        } else {
            Err(StorageClientError::ReadSmallFileError)
        };
    }

    pub async fn read_large_file(
        &self,
        user_id: Uuid,
        filename: &str,
    ) -> Result<Vec<u8>, StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let remote_file = main_server_client
            .get_large_file(GetLargeFileRequest {
                filename: filename.to_string(),
                user_id: user_id.as_bytes().to_vec(),
                group_ids: vec![],
            })
            .await
            .map_err(|e| StorageClientError::ReadSmallFileError)?
            .into_inner();

        let count = remote_file.blocks.len();
        let mut data: Vec<Vec<u8>> = (0..count).map(|_| vec![]).collect();

        for block in remote_file.blocks {
            let mut data_node_client =
                DataNodeServiceApiClient::connect(format!("http://{}", block.endpoint))
                    .await
                    .map_err(|_| StorageClientError::WrongDatanodeAddressError)?;

            let response = data_node_client
                .read_block(ReadBlockRequest {
                    part: block.part,
                    block_id: block.block_id,
                })
                .await;

            let mut local_data = vec![];

            if let Ok(stream) = response {
                let mut stream = stream.into_inner();
                while let Some(part) = stream.next().await {
                    if let Ok(part) = part {
                        local_data.push(part.data);
                    } else {
                        return Err(StorageClientError::ReadLargeFileError);
                    }
                }
                data[block.part as usize] = local_data.into_iter().flatten().collect()
            } else {
                return Err(StorageClientError::ReadLargeFileError);
            }
        }

        return Ok(data.into_iter().flatten().collect());
    }

    pub async fn delete_file(
        &self,
        user_id: Uuid,
        filename: &str,
    ) -> Result<(), StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let _ = main_server_client
            .delete_file(DeleteFileRequest {
                filename: filename.to_string(),
                user_id: user_id.as_bytes().to_vec(),
                group_ids: vec![],
            })
            .await
            .map_err(|e| StorageClientError::DeleteFileError)?;

        Ok(())
    }

    pub async fn add_new_commit_to_small_file(
        &self,
        user_id: Uuid,
        filename: &str,
        data: &[u8],
    ) -> Result<(), StorageClientError> {
        let mut main_server_client = MainServerServiceApiClient::connect(format!(
            "http://{}",
            self.config.main_server_address
        ))
        .await
        .map_err(|_| StorageClientError::WrongMetadataAddressError)?;

        let block = main_server_client
            .add_commit_to_small_file(AddCommitSmallFileRequest {
                filename: filename.to_string(),
                user_id: user_id.to_bytes_le().to_vec(),
                group_ids: vec![],
            })
            .await
            .map_err(|_| StorageClientError::AddNewCommitToSmallFileError)?
            .into_inner();

        let mut data_node_client =
            DataNodeServiceApiClient::connect(format!("http://{}", block.endpoint))
                .await
                .map_err(|_| StorageClientError::WrongDatanodeAddressError)?;

        let update_info = UpdateBlockRequest {
            filename: filename.to_string().clone(),
            block_id: block.block_id.clone(),
            part: block.part,
            range: Some(Range {
                start: 0,
                end: data.len() as u64,
            }),
            data: data.to_vec(),
            hash: 0,
        };

        let stream = tokio_stream::iter(std::iter::once(update_info));

        let _ = data_node_client
            .update_block(stream)
            .await
            .map_err(|_| StorageClientError::UpdateBlockError)?
            .into_inner();

        Ok(())
    }
}

#[derive(Debug)]
pub enum StorageClientError {
    WrongMetadataAddressError,
    WrongDatanodeAddressError,
    CreateFileError,
    UpdateBlockError,
    ReadSmallFileError,
    ReadLargeFileError,
    DeleteFileError,
    AddNewCommitToSmallFileError,
}
