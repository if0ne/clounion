use super::large_file::LargeFile;
use super::small_file::SmallFile;
use fast_str::FastStr;
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

#[derive(Clone, Serialize, Deserialize)]
pub enum ObjectVariant<T, Hash>
where
    T: Serialize,
    Hash: Serialize + Copy + AsBytes,
{
    LargeFile(LargeFile<T, Hash>),
    SmallFile(SmallFile<T, Hash>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Object<T, Hash>
where
    T: Serialize,
    Hash: Serialize + Copy + AsBytes,
{
    name: FastStr,
    size: usize,
    inner: ObjectVariant<T, Hash>,
}

impl<T, Hash> Object<T, Hash>
where
    T: Serialize,
    Hash: Serialize + Copy + AsBytes,
{
    pub fn new(name: FastStr, size: usize, inner: ObjectVariant<T, Hash>) -> Self {
        Self { name, size, inner }
    }
}
