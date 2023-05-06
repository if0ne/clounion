use super::large_file::LargeFile;
use super::small_file::SmallFile;
use fast_str::FastStr;
use redis::{FromRedisValue, RedisResult, Value};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zerocopy::AsBytes;

#[derive(Clone, Serialize, Deserialize)]
pub enum ObjectVariant<T>
where
    T: Serialize,
{
    LargeFile(LargeFile<T>),
    SmallFile(SmallFile<T>),
}

impl<T> ObjectVariant<T>
where
    T: Serialize,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct Object<T>
where
    T: Serialize,
{
    pub(crate) name: FastStr,
    pub(crate) size: usize,
    pub(crate) inner: ObjectVariant<T>,
}

impl<T> Object<T>
where
    T: Serialize,
{
    pub fn new(name: FastStr, size: usize, inner: ObjectVariant<T>) -> Self {
        Self { name, size, inner }
    }

    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        self.inner.update_block(block_id, part, checksum);
    }
}
