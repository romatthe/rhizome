extern crate bincode;

use hash::KeyHash;
use nodeid::NodeId;
use node::UDP_SOCKET_BUFFER_BYTES;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Message {
    pub payload: MessagePayload,
    pub origin: NodeId
}

impl Message {
    pub fn serialize(&self) -> Vec<u8> {
        bincode::serialize(&self, bincode::Bounded(UDP_SOCKET_BUFFER_BYTES)).unwrap()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum MessagePayload {
    Ping,
    Pong,
    Store,
    StoreResponse,
    FindNode,
    FindNodeResponse,
    FindValue,
    FindValueResponse
}

pub struct StoreMessage {
    pub key: KeyHash,
    pub data: Vec<u8>
}

pub struct StoreResponseMessage {

}

pub struct FindNodeMessage {

}

pub struct FindNodeResponseMessage {

}

pub struct FindValueMessage {
    pub key: KeyHash
}

pub struct FindValueResponseMessage {

}