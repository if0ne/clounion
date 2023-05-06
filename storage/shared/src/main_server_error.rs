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
}

impl Display for MetadataError {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl_converter!(MetadataError);
