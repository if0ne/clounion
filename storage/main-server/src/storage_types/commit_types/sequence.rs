use super::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Sequence<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    pub(crate) seq: Vec<Block<T, Hash>>,
}
