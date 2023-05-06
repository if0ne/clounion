use crate::constants::MAX_REPLICAS;
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use uuid::Uuid;

#[derive(Clone, Serialize, Deserialize)]
pub struct Block<Dst, Hash>
where
    Dst: Serialize,
    Hash: Serialize,
{
    pub(crate) id: Uuid,
    pub(crate) part: usize,
    pub(crate) dst: Dst,
    pub(crate) replicas: Vec<[Dst; MAX_REPLICAS]>,
    pub(crate) checksum: Hash,
}
