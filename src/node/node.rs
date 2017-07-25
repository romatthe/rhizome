extern crate rand;

use std::net::{SocketAddr, Ipv4Addr, Ipv6Addr, UdpSocket};
use hash::HASH_SIZE;
use nodeid::NodeId;
use routing::RoutingTable;

pub const UDP_SOCKET_BUFFER_BYTES: u64 = 65508;

pub struct Node {
    pub contact: Contact,
    pub routing: RoutingTable,
    pub sock_in: UdpSocket,
    pub sock_out: UdpSocket
}

pub struct Contact {
    pub id: NodeId,
    pub ip: SocketAddr,
}

impl Node {
    pub fn new() -> Node {
        let ip = "0.0.0.0:0".parse().unwrap();
        Node {
            contact: Contact { id: NodeId::new(), ip: ip },
            routing: RoutingTable::new(),
            sock_in: UdpSocket::bind(ip).expect("Failed to bind socket!"),
            sock_out: UdpSocket::bind(ip).expect("Failed to bind socket!")
        }
    }

    pub fn distance(&self, node: &Node) -> NodeId {
        &self.contact.id ^ &node.contact.id
    }

    pub fn distance2(&self, node: &Node) -> u32 {
//        return ID_LENGTH - this.xor(to).getFirstSetBitIndex();
//        let zeroes = self.distance(node).count_leading_zeroes() as usize;
        (HASH_SIZE as u32) - self.distance(node).count_leading_zeroes()
    }
}