mod proto_data_node {
    tonic::include_proto!("data_node");
}

use crate::block_storage_service::BlockStorageService;
use crate::data_node_controller::proto_data_node::{
    data_node_service_server::{DataNodeService, DataNodeServiceServer},
    BlockInfo, CreateBlocksRequest, CreateBlocksResponse, DeleteBlockRequest, EmptyResponse,
    ReadBlockRequest, ReadBlockResponse, UpdateBlockRequest, UpdateBlockResponse,
};
use crate::data_node_info::DataNodeInfo;
use crate::main_server_client::MainServerClient;
use shared::data_node_error::DataNodeError;
use std::sync::Arc;
use tokio_stream::wrappers::ReceiverStream;
use tokio_stream::StreamExt;
use tonic::{Request, Response, Status, Streaming};
use uuid::Uuid;

pub struct DataNodeController {
    block_storage_service: Arc<BlockStorageService>,
    main_server_client: MainServerClient,
}

impl DataNodeController {
    pub async fn get_service(
        data_node_info: DataNodeInfo,
        main_server_client: MainServerClient,
    ) -> std::io::Result<DataNodeServiceServer<Self>> {
        let block_storage_service = BlockStorageService::new(data_node_info).await?;

        Ok(DataNodeServiceServer::new(Self {
            block_storage_service: Arc::new(block_storage_service),
            main_server_client,
        }))
    }
}

#[tonic::async_trait]
impl DataNodeService for DataNodeController {
    async fn create_blocks(
        &self,
        request: Request<CreateBlocksRequest>,
    ) -> Result<Response<CreateBlocksResponse>, Status> {
        let blocks = self
            .block_storage_service
            .create_blocks(request.get_ref().count as usize)
            .await?
            .into_iter()
            .map(|el| BlockInfo {
                block_id: el.1.as_bytes().to_vec(),
                part: el.0 as u64,
            })
            .collect();
        Ok(Response::new(CreateBlocksResponse {
            blocks,
            endpoint: self.block_storage_service.get_endpoint(),
        }))
    }

    type ReadBlockStream = ReceiverStream<Result<ReadBlockResponse, Status>>;

    async fn read_block(
        &self,
        request: Request<ReadBlockRequest>,
    ) -> Result<Response<Self::ReadBlockStream>, Status> {
        let inner = request.into_inner();

        let uuid = Uuid::from_slice(&inner.block_id)
            .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", &inner.block_id)))?;
        let part = inner.part;

        let (controller_tx, controller_rx) = tokio::sync::mpsc::channel(128);
        let (service_tx, service_rx) = tokio::sync::mpsc::channel(128);
        let block_storage = self.block_storage_service.clone();

        tokio::spawn(async move {
            let response = block_storage
                .read_block(uuid, part as usize, service_tx)
                .await;

            if let Err(err) = response {
                let response = controller_tx.send(Err(err.into())).await;
                if let Err(err) = response {
                    tracing::debug!("Send error while reading: {}", err);
                    return;
                }
            }

            let mut stream = ReceiverStream::new(service_rx);
            while let Some(response) = stream.next().await {
                let response = controller_tx
                    .send(
                        response
                            .map(|buffer| ReadBlockResponse {
                                size: buffer.len() as u64,
                                data: buffer,
                            })
                            .map_err(|err| err.into()),
                    )
                    .await;

                match response {
                    Ok(_) => {
                        tracing::debug!(
                            "Sent data chunk for {}",
                            Uuid::from_slice(&inner.block_id).unwrap()
                        );
                    }
                    Err(_) => {
                        tracing::error!(
                            "Stream for {} was dropped",
                            Uuid::from_slice(&inner.block_id).unwrap()
                        );
                        break;
                    }
                }
            }
        });

        Ok(Response::new(ReceiverStream::new(controller_rx)))
    }

    async fn update_block(
        &self,
        request: Request<Streaming<UpdateBlockRequest>>,
    ) -> Result<Response<UpdateBlockResponse>, Status> {
        let mut inner = request.into_inner();

        //Unsafe
        let mut block_id = Uuid::nil();
        let mut block_part = 0;
        let mut filename = String::new();

        while let Some(message) = inner.message().await? {
            let Ok(uuid) = Uuid::from_slice(&message.block_id) else {
                return Err(DataNodeError::WrongUuid(format!("{:?}", &message.block_id)).into());
            };

            let part = message.part;

            block_id = uuid;
            block_part = part as usize;
            filename = message.filename;

            if let Some(range) = message.range {
                let range = (range.start as usize)..(range.end as usize);
                let data = message.data;

                let Ok(_) = self.block_storage_service.update_block(uuid, part as usize, range, &data).await else {
                    return Err(DataNodeError::UpdateBlockError(format!("{:?}", uuid)).into());
                };
            } else {
                tracing::error!("Range are null");
                todo!("Return error")
            }
        }

        let checksum = self
            .block_storage_service
            .get_block_checksum(block_id, block_part)
            .await?;

        self.main_server_client
            .add_checksum(
                &filename,
                crate::main_server_client::proto_main_server::BlockInfo {
                    block_id: block_id.to_bytes_le().to_vec(),
                    part: block_part as u64,
                    endpoint: self.block_storage_service.get_endpoint(),
                },
                checksum,
            )
            .await;

        Ok(Response::new(UpdateBlockResponse {}))
    }

    async fn delete_block(
        &self,
        request: Request<DeleteBlockRequest>,
    ) -> Result<Response<EmptyResponse>, Status> {
        let inner = request.into_inner();

        if let Some(block) = inner.block {
            let uuid = Uuid::from_slice(&block.block_id)
                .map_err(|_| DataNodeError::WrongUuid(format!("{:?}", &block.block_id)))?;
            let part = block.part as usize;

            self.block_storage_service.delete_block(uuid, part).await?;
        } else {
            tracing::error!("Block are null");
            todo!("Return error")
        }

        Ok(Response::new(EmptyResponse {}))
    }
}
