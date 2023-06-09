use super::large_file::LargeFile;
use super::small_file::SmallFile;
use crate::storage_types::commit_types::block::Block;
use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ObjectVariant<T>
where
    T: Serialize + Debug,
{
    LargeFile(LargeFile<T>),
    SmallFile(SmallFile<T>),
}

impl<T> ObjectVariant<T>
where
    T: Serialize + Debug,
{
    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        match self {
            ObjectVariant::LargeFile(file) => {
                file.update_block(block_id, part, checksum);
            }
            ObjectVariant::SmallFile(file) => {
                file.update_block(block_id, part, checksum);
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Object<T>
where
    T: Serialize + Debug,
{
    pub(crate) name: FastStr,
    pub(crate) size: usize,
    pub(crate) owner: Uuid,
    pub(crate) inner: ObjectVariant<T>,
}

impl<T> Object<T>
where
    T: Serialize + Debug,
{
    pub fn new(name: FastStr, size: usize, owner: Uuid, inner: ObjectVariant<T>) -> Self {
        Self {
            name,
            size,
            owner,
            inner,
        }
    }

    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        self.inner.update_block(block_id, part, checksum);
    }

    pub fn get_all_blocks(&self) -> &[Block<T, u32>] {
        return match self.inner {
            ObjectVariant::LargeFile(ref file) => file.get_all_blocks(),
            ObjectVariant::SmallFile(ref file) => file.get_all_blocks(),
        };
    }
}
