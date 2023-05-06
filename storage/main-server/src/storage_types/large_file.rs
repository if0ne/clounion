use super::commit_types::block::Block;
use super::commit_types::merkle_tree::MerkleTree;
use serde::{Deserialize, Serialize};
use zerocopy::AsBytes;

#[derive(Clone, Serialize, Deserialize)]
pub struct LargeFile<T, Hash>
where
    T: Serialize,
    Hash: Serialize + Copy + AsBytes,
{
    pub(crate) parts: MerkleTree<T, Hash>,
}
