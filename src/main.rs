extern crate arrayvec;
extern crate rand;
extern crate sha1;

mod node;
mod routingtable;

use node::Node;
use routingtable::RoutingTable;

fn main() {
    let new_node = Node::new();
    println!("New Node id acquired: {}", new_node.id);
    let new_node2 = Node::new();
    println!("New Node id acquired: {}", new_node2.id);
    println!("Distance betweeen two nodes: {}", new_node.distance(new_node2));

    println!("New routintable: {:?}", RoutingTable::new().bucket_at(0));
}

