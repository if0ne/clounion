use super::sequence::Sequence;

pub enum Commits<T> {
    Sequence(Sequence<T>),
}
