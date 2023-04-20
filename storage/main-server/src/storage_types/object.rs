use super::large_file::LargeFile;
use super::small_file::SmallFile;
use fast_str::FastStr;

pub enum ObjectVariant<T> {
    LargeFile(LargeFile<T>),
    SmallFile(SmallFile<T>),
}

pub struct Object<T, Hash> {
    name: FastStr,
    inner: ObjectVariant<T>,
    checksum: Hash
}

