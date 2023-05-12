use crate::impl_converter;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum DataNodeError {
    CreateBlocksError(String),
    CreateBlockError(String),
    BlockNotFound(String),
    WrongUuid(String),
    ReadBlockError(String),
    UpdateBlockError(String),
    DeleteBlockError(String),
    NoSpace,
    BlockOverflow(usize, usize),
}

impl_converter!(DataNodeError);

impl std::fmt::Display for DataNodeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DataNodeError::CreateBlocksError(str) => {
                write!(f, "Fail to create blocks. {0}", str)
            }
            DataNodeError::CreateBlockError(str) => {
                write!(f, "Fail to create block {0}", str)
            }
            DataNodeError::BlockNotFound(str) => {
                write!(f, "Block {0} was not found", str)
            }
            DataNodeError::WrongUuid(str) => {
                write!(f, "Got wrong uuid format {0}", str)
            }
            DataNodeError::ReadBlockError(str) => {
                write!(f, "Fail to read block {0}", str)
            }
            DataNodeError::UpdateBlockError(str) => {
                write!(f, "Fail to update block {0}", str)
            }
            DataNodeError::DeleteBlockError(str) => {
                write!(f, "Fail to delete block {0}", str)
            }
            DataNodeError::NoSpace => {
                write!(f, "No space")
            }
            DataNodeError::BlockOverflow(block_size, buffer_size) => {
                write!(
                    f,
                    "Trying to write {} bytes in block of {} bytes size",
                    buffer_size, block_size
                )
            }
        }
    }
}
