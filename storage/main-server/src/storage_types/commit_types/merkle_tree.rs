use crate::storage_types::commit_types::block::Block;
use tokio::sync::RwLock;

#[derive(Clone)]
enum Node<Dst, Hash> {
    Node {
        left: Option<Box<Node<Dst, Hash>>>,
        right: Option<Box<Node<Dst, Hash>>>,
        checksum: Hash,
    },
    Leaf(Block<Dst, Hash>),
}

#[derive(Clone)]
pub struct MerkleTree<Dst, Hash> {
    root: Node<Dst, Hash>,
}
