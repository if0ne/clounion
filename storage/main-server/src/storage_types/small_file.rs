use super::commit_types::commit::Commits;
use crate::storage_types::commit_types::block::Block;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SmallFile<T>
where
    T: Serialize,
{
    pub(crate) commits: Commits<T>,
}

impl<T> SmallFile<T>
where
    T: Serialize,
{
    pub fn add_block(&mut self, block: Block<T, u32>) {
        match self.commits {
            Commits::Sequence(ref mut seq) => {
                seq.seq.push(block);
            }
        }
    }

    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        match self.commits {
            Commits::Sequence(ref mut seq) => {
                seq.update_block(block_id, part, checksum);
            }
        }
    }

    pub fn get_all_blocks(&self) -> &[Block<T, u32>] {
        match self.commits {
            Commits::Sequence(ref seq) => &seq.seq,
        }
    }
}
