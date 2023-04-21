use super::commit_types::commit::Commits;
use fast_str::FastStr;

#[derive(Clone)]
pub struct SmallFile<T, Hash> {
    pub(crate) commits: Commits<T, Hash>,
}
