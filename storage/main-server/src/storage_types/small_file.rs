use super::commit_types::commit::Commits;
use fast_str::FastStr;

pub struct SmallFile<T> {
    commits: Commits<T>,
}
