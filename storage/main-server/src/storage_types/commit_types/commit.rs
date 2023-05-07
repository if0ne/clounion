use super::sequence::Sequence;
use crate::storage_types::commit_types::block::Block;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Commits<T>
where
    T: Serialize,
{
    Sequence(Sequence<T>),
}

impl<T> Commits<T>
where
    T: Serialize,
{
    pub fn last(&self) -> &Block<T, u32> {
        return match self {
            Commits::Sequence(seq) => seq.seq.last().unwrap(),
        };
    }

    pub fn index(&self, index: usize) -> &Block<T, u32> {
        return match self {
            Commits::Sequence(seq) => &seq.seq[index],
        };
    }
}
