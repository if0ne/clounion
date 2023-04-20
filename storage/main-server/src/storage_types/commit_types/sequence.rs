use super::block::Block;

pub struct Sequence<T> {
    seq: Vec<Block<T>>,
}
