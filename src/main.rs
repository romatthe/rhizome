extern crate arrayvec;
extern crate bincode;
extern crate byteorder;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate sha1;

mod hash;
mod node;
mod routingtable;
mod rpc;

use node::Node;
use routingtable::RoutingTable;
use rpc::{MessagePayload, Message};

fn main() {
    let new_node = Node::new();
    println!("New Node id acquired: {}", new_node.contact.id);
    let new_node2 = Node::new();
    println!("New Node id acquired: {}", new_node2.contact.id);
    println!("Distance between two nodes: {:b}", new_node.distance(&new_node2));

    println!("New routingtable: {:?}", RoutingTable::new().bucket_at(0));

    let payload: MessagePayload = MessagePayload::Ping;
    let message: Message = Message { payload: payload, origin: new_node2.contact.id };

    println!("New Rpc message - Deserialized: {:?}", message);
    println!("New Rpc message - Serialized: {:?}", message.serialize());
}

