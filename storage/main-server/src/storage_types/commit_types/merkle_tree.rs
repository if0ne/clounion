use crate::storage_types::commit_types::block::Block;
use serde::{Deserialize, Serialize};
use std::hash::Hash;
use futures::StreamExt;
use tokio::sync::RwLock;
use zerocopy::AsBytes;

#[derive(Clone, Serialize, Deserialize)]
struct Node<Hash>
where
    Hash: Serialize + AsBytes + Copy,
{
    left: usize,
    right: usize,
    pub(crate) checksum: Hash,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MerkleTree<Dst, Hash>
where
    Dst: Serialize,
    Hash: Serialize + Copy + AsBytes,
{
    leafs: Vec<Block<Dst, Hash>>,
    nodes: Vec<Vec<Node<Hash>>>,
}

impl<Dst> MerkleTree<Dst, u32>
where
    Dst: Serialize
{
    pub fn build(blocks: Vec<Block<Dst, u32>>) -> Self {
        let mut offset = 0;
        let mut nodes = vec![];

        let chunks = blocks.chunks(2);
        let len = chunks.len();
        let mut inner_nodes = vec![];

        for (i, chunk) in chunks.enumerate() {
            if chunk.len() == 2 {
                let checksum = {
                    let vec = [chunk[0].checksum, chunk[1].checksum];
                    let bytes = vec.as_bytes();

                    //TODO: Ломает всю абстракцию выбора хеша
                    crc32fast::hash(&bytes)
                };

                inner_nodes.push(Node {
                    left: offset + i * 2,
                    right: offset + i * 2 + 1,
                    checksum
                });
            } else {
                inner_nodes.push(Node {
                    left: offset + i * 2,
                    right: offset + i * 2,
                    checksum: chunk[0].checksum,
                });
            }


        }

        nodes.push(inner_nodes);
        offset += len;

        let mut prev = 0;
        while nodes[prev].len() != 1 {
            let chunks = nodes[prev].chunks(2);
            let len = chunks.len();
            let mut inner_nodes = vec![];

            for (i, chunk) in chunks.enumerate() {
                if chunk.len() == 2 {
                    let checksum = {
                        let vec = [chunk[0].checksum, chunk[1].checksum];
                        let bytes = vec.as_bytes();

                        //TODO: Ломает всю абстракцию выбора хеша
                        crc32fast::hash(&bytes)
                    };

                    inner_nodes.push(Node {
                        left: offset + i * 2,
                        right: offset + i * 2 + 1,
                        checksum
                    });
                } else {
                    inner_nodes.push(Node {
                        left: offset + i * 2,
                        right: offset + i * 2,
                        checksum: chunk[0].checksum,
                    });
                }
            }

            nodes.push(inner_nodes);
            offset += len;
            prev += 1;
        }

        Self {
            leafs: blocks,
            nodes,
        }
    }

    pub fn root(&self) -> u32 {
        self.nodes.last().unwrap()[0].checksum
    }
}
