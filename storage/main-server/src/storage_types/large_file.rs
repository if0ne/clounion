use super::commit_types::block::Block;
use super::commit_types::merkle_tree::MerkleTree;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct LargeFile<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    parts: Vec<MerkleTree<T, Hash>>,
}
