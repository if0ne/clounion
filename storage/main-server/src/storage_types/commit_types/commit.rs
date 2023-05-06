use super::sequence::Sequence;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Commits<T>
where
    T: Serialize,
{
    Sequence(Sequence<T>),
}
