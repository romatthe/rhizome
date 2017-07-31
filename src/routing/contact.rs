use std::net::{SocketAddr};
use std::time::SystemTime;
use node::NodeId;

#[derive(Debug, PartialEq)]
pub struct Contact {
    pub id: NodeId,
    pub ip: SocketAddr,
    pub last_seen: SystemTime
}