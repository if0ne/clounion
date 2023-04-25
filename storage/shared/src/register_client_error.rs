use crate::impl_converter;
use serde::{Deserialize, Serialize};
use std::fmt::Formatter;

#[derive(Debug, Serialize, Deserialize)]
pub enum RegistryError {
    WrongBlockSize(usize, usize, usize),
}

impl_converter!(RegistryError);

impl std::fmt::Display for RegistryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RegistryError::WrongBlockSize(got, small, large) => {
                write!(
                    f,
                    "Got block size {} bytes, but server can handle in block size with {} or {} bytes.",
                    got, small, large
                )
            }
        }
    }
}
