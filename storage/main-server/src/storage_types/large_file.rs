use super::commit_types::block::Block;
use super::commit_types::merkle_tree::MerkleTree;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LargeFile<T>
where
    T: Serialize + Debug,
{
    pub(crate) tree: MerkleTree<T, u32>,
}

impl<T> LargeFile<T>
where
    T: Serialize + Debug,
{
    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        self.tree.update_block(block_id, part, checksum);
    }

    pub fn get_all_blocks(&self) -> &[Block<T, u32>] {
        self.tree.leaves()
    }
}
