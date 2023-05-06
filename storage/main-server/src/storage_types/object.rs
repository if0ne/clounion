use super::large_file::LargeFile;
use super::small_file::SmallFile;
use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum ObjectVariant<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    LargeFile(LargeFile<T, Hash>),
    SmallFile(SmallFile<T, Hash>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Object<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    name: FastStr,
    size: usize,
    inner: ObjectVariant<T, Hash>,
}

impl<T, Hash> Object<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    pub fn new(name: FastStr, size: usize, inner: ObjectVariant<T, Hash>) -> Self {
        Self { name, size, inner }
    }
}
