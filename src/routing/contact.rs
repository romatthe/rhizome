use std::net::{SocketAddr};
use node::NodeId;

pub struct Contact {
    pub id: NodeId,
    pub ip: SocketAddr,
}