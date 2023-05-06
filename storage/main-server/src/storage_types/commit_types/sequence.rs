use super::block::Block;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Sequence<T>
where
    T: Serialize,
{
    pub(crate) seq: Vec<Block<T, u32>>,
}

impl<T> Sequence<T>
where
    T: Serialize,
{
    pub fn update_block(&mut self, block_id: Uuid, part: usize, checksum: u32) {
        for block in self.seq.iter_mut() {
            if block.id == block_id && block.part == part {
                block.checksum = checksum;
                return;
            }
        }
    }
}
