use crate::constants::MAX_REPLICAS;
use smallvec::SmallVec;
use uuid::Uuid;

pub struct Block<T> {
    id: Uuid,
    dst: T,
    replicas: SmallVec<[T; MAX_REPLICAS]>,
}
