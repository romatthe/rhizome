extern crate bincode;

use node::NodeId;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct RpcMessage {
    pub payload: Rpc,
    pub origin: NodeId
}

impl RpcMessage {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self, bincode::Bounded(2048)).unwrap() // Fix the size limit to the UDP datagram size
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum Rpc {
    Ping,
    PingResponse,
    Store,
    StoreResponse,
    FindNode,
    FindNodeResponse,
    FindValue,
    FindValueResponse
}

pub struct StoreMessage {

}

pub struct StoreResponseMessage {

}

pub struct FindNodeMessage {

}

pub struct FindNodeResponseMessage {

}

pub struct FindValueMessage {

}

pub struct FindValueResponseMessage {

}