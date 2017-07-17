extern crate bincode;

use node::NodeId;
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