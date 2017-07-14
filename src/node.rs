extern crate rand;
extern crate sha1;

use rand::Rng;
use std::fmt;

pub struct Node {
    pub id: NodeId
}

pub struct NodeId {
    pub value: Vec<u8>
}

impl NodeId {
    fn new() -> NodeId {
        let mut rng = rand::thread_rng();
        let id = rng.gen_iter::<u8>().take(20).collect::<Vec<u8>>();

        NodeId {
            value: id
        }
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut m = sha1::Sha1::new();
        m.update(self.value.as_ref());
        write!(f, "{}", m.digest())
    }
}

impl Node {
    pub fn new() -> Node {
        Node {
            id: NodeId::new()
        }
    }
}