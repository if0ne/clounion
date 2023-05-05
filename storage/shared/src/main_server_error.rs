use crate::impl_converter;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub enum MetadataError {
    CreateSmallFileError(String),
    CreateBlocksResponseError(String),
}

impl Display for MetadataError {
    fn fmt(&self, _: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl_converter!(MetadataError);
