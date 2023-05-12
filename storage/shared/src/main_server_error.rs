use crate::impl_converter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum MetadataError {
    CreateFileError(String),
    CreateBlocksResponseError(String),
    FileNotFoundError(String),
    CannotAddBlockToLargeFileError(String),
    TryingToGetSmallButItLarge(String),
    TryingToGetLargeButItSmall(String),
    WrongUuid(String),
}

impl Display for MetadataError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MetadataError::CreateFileError(msg) => {
                write!(f, "Fail to create small file. {0}", msg)
            }
            MetadataError::CreateBlocksResponseError(msg) => {
                write!(f, "Blocks response error. {0}", msg)
            }
            MetadataError::FileNotFoundError(msg) => {
                write!(f, "File not found. {0}", msg)
            }
            MetadataError::CannotAddBlockToLargeFileError(msg) => {
                write!(f, "Can not add block to large file. {0}", msg)
            }
            MetadataError::TryingToGetSmallButItLarge(msg) => {
                write!(f, "Trying to get small file but it large. {0}", msg)
            }
            MetadataError::TryingToGetLargeButItSmall(msg) => {
                write!(f, "Trying to get large file but it small. {0}", msg)
            }
            MetadataError::WrongUuid(msg) => {
                write!(f, "Got wrong uuid format {0}", msg)
            }
        }
    }
}

impl_converter!(MetadataError);
