use super::commit_types::commit::Commits;
use fast_str::FastStr;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct SmallFile<T, Hash>
where
    T: Serialize,
    Hash: Serialize,
{
    pub(crate) commits: Commits<T, Hash>,
}
