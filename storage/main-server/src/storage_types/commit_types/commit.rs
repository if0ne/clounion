use super::sequence::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Commits<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    Sequence(Sequence<T, Hash>),
}
