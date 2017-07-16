extern crate rand;

use std::fmt;
use std::net::UdpSocket;
use std::ops::BitXor;
use rand::Rng;
use routingtable::RoutingTable;

pub const HASH_SIZE: usize = 160;
pub const HASH_SIZE_BYTES: usize = HASH_SIZE / 8;

pub struct Node {
    pub id: NodeId,
    pub routing: RoutingTable,
    pub sock_in: UdpSocket,
    pub sock_out: UdpSocket
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NodeId {
    pub value: Vec<u8>
}

impl NodeId {
    pub fn new() -> NodeId {
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
            id: NodeId::new(),
            routing: RoutingTable::new(),
            sock_in: UdpSocket::bind(("0.0.0.0", 0)).expect("Failed to bind socket!"),
            sock_out: UdpSocket::bind(("0.0.0.0", 0)).expect("Failed to bind socket!")
        }
    }

    pub fn distance(self, node: &Node) -> NodeId {
        self.id ^ &node.id
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = &self.value.iter().fold(String::from(""), |acc, &byte| format!("{}{:08b}", &acc, &byte));
        write!(f, "{}", fmt)
    }
}

impl <'a> BitXor<&'a NodeId> for NodeId {
    type Output = Self;

    fn bitxor (self, rhs: &Self) -> NodeId {
        let dist = self.value.iter()
            .zip(rhs.value.iter())
            .map(|(byte1, byte2)| byte1 ^ byte2)
            .collect::<Vec<u8>>();

        NodeId { value: dist }
    }
}