extern crate rand;

use std::fmt;
use std::io::Cursor;
use std::net::{SocketAddr, Ipv4Addr, Ipv6Addr, UdpSocket};
use std::ops::BitXor;
use byteorder::{BigEndian, ReadBytesExt};
use rand::Rng;
use hash::HASH_SIZE_BYTES;
use routingtable::RoutingTable;

pub const UDP_SOCKET_BUFFER_BYTES: u64 = 65508;

pub struct Node {
    pub contact: Contact,
    pub routing: RoutingTable,
    pub sock_in: UdpSocket,
    pub sock_out: UdpSocket
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NodeId {
    pub value: Vec<u8>
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

    pub fn distance(self, node: &Node) -> NodeId {
        self.contact.id ^ &node.contact.id
    }
}

impl NodeId {
    pub fn new() -> NodeId {
        let mut rng = rand::thread_rng();
        let id = rng.gen_iter::<u8>().take(HASH_SIZE_BYTES).collect::<Vec<u8>>();

        NodeId { value: id }
    }

    pub fn from_bytes(bytes: &[u8; 20]) -> NodeId {
        NodeId { value: bytes.to_vec() }
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rdr = Cursor::new(&self.value);
        write!(f, "{:?}", rdr.read_u16::<BigEndian>().unwrap())
    }
}

impl fmt::Binary for NodeId {
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