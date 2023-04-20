use super::commit_types::block::Block;

pub struct LargeFile<T> {
    parts: Vec<Block<T>>,
}
