use super::large_file::LargeFile;
use super::small_file::SmallFile;
use fast_str::FastStr;

#[derive(Clone)]
pub enum ObjectVariant<T, Hash> {
    LargeFile(LargeFile<T, Hash>),
    SmallFile(SmallFile<T, Hash>),
}

#[derive(Clone)]
pub struct Object<T, Hash> {
    name: FastStr,
    inner: ObjectVariant<T, Hash>,
}

impl<T, Hash> Object<T, Hash> {
    pub fn new(name: FastStr, inner: ObjectVariant<T, Hash>) -> Self {
        Self { name, inner }
    }
}
