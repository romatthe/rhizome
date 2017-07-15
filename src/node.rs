extern crate rand;
extern crate sha1;

use std::fmt;
use std::ops::BitXor;
use rand::Rng;

pub const HASH_SIZE: usize = 160;
pub const HASH_SIZE_BYTES: usize = HASH_SIZE / 8;

pub struct Node {
    pub id: NodeId
}

#[derive(Debug)]
pub struct NodeId {
    pub value: Vec<u8>
}

impl NodeId {
    fn new() -> NodeId {
        let mut rng = rand::thread_rng();
        let id = rng.gen_iter::<u8>().take(HASH_SIZE_BYTES).collect::<Vec<u8>>();

        NodeId {
            value: id
        }
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: NodeId::new()
        }
    }

    pub fn distance(self, node: Node) -> NodeId {
        self.id ^ node.id
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m = sha1::Sha1::new();
        m.update(self.value.as_ref());
        write!(f, "{}", m.digest())
    }
}

impl BitXor<NodeId> for NodeId {
    type Output = Self;

    fn bitxor (self, rhs: Self) -> NodeId {
        let dist = self.value.iter()
            .zip(rhs.value.iter())
            .map(|(byte1, byte2)| byte1 ^ byte2)
            .collect::<Vec<u8>>();

        NodeId { value: dist }
    }
}