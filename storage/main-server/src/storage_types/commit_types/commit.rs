use super::sequence::Sequence;

#[derive(Clone)]
pub enum Commits<T, Hash> {
    Sequence(Sequence<T, Hash>),
}
