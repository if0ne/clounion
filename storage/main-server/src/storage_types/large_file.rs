use super::commit_types::block::Block;
use super::commit_types::merkle_tree::MerkleTree;

#[derive(Clone)]
pub struct LargeFile<T, Hash> {
    parts: Vec<MerkleTree<T, Hash>>,
}
