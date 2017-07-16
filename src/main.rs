extern crate arrayvec;
extern crate bincode;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate sha1;

mod node;
mod routingtable;
mod rpc;

use node::Node;
use routingtable::RoutingTable;
use rpc::{Rpc, RpcMessage};

fn main() {
    let new_node = Node::new();
    println!("New Node id acquired: {}", new_node.id);
    let new_node2 = Node::new();
    println!("New Node id acquired: {}", new_node2.id);
    println!("Distance betweeen two nodes: {}", new_node.distance(&new_node2));

    println!("New routintable: {:?}", RoutingTable::new().bucket_at(0));

    let payload: Rpc = Rpc::Ping;
    let message: RpcMessage = RpcMessage{ payload: payload, origin: new_node2.id };

    println!("New Rpc message - Deserialized: {:?}", message);
    println!("New Rpc message - Serialized: {:?}", message.serialize());
}

