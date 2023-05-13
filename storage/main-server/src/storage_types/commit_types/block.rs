use crate::constants::MAX_REPLICAS;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Block<Dst, Hash>
where
    Dst: Serialize,
    Hash: Serialize + Debug,
{
    pub(crate) id: Uuid,
    pub(crate) part: usize,
    pub(crate) dst: Dst,
    pub(crate) replicas: Vec<[Dst; MAX_REPLICAS]>,
    pub(crate) checksum: Hash,
}
