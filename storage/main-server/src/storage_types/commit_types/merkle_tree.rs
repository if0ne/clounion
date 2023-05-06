use crate::storage_types::commit_types::block::Block;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Clone, Serialize, Deserialize)]
enum Node<Dst, Hash>
where
    Dst: Serialize,
    Hash: Serialize,
{
    Node {
        left: Option<Box<Node<Dst, Hash>>>,
        right: Option<Box<Node<Dst, Hash>>>,
        checksum: Hash,
    },
    Leaf(Block<Dst, Hash>),
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MerkleTree<Dst, Hash>
where
    Dst: Serialize,
    Hash: Serialize,
{
    root: Node<Dst, Hash>,
}
