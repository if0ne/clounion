use super::block::Block;

#[derive(Clone)]
pub struct Sequence<T, Hash> {
    pub(crate) seq: Vec<Block<T, Hash>>,
}
