extern crate rand;

mod node_id;

pub use self::node_id::NodeId;

use std::net::{SocketAddr, Ipv4Addr, Ipv6Addr, UdpSocket};
use hash::HASH_SIZE;
use routing::{Contact, RoutingTable};

pub const UDP_SOCKET_BUFFER_BYTES: u64 = 65508;

pub struct Node {
    pub contact: Contact,
    pub routing: RoutingTable,
    pub sock_in: UdpSocket,
    pub sock_out: UdpSocket
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

    pub fn distance(&self, node: &Node) -> u32 {
        let binary_distance = &self.contact.id ^ &node.contact.id;
        (HASH_SIZE as u32) - binary_distance.count_leading_zeroes()
    }
}