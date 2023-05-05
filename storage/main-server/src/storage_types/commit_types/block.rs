use crate::constants::MAX_REPLICAS;
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Clone)]
pub struct Block<Dst, Hash> {
    pub(crate) id: Uuid,
    pub(crate) part: usize,
    pub(crate) dst: Dst,
    pub(crate) replicas: SmallVec<[Dst; MAX_REPLICAS]>,
    pub(crate) checksum: Hash,
}
