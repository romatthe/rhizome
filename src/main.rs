extern crate rand;
extern crate sha1;

mod node;

use node::Node;

fn main() {
    println!("Hello, world!");
    let new_node = Node::new();
    println!("New Node id acquired: {}", new_node.id);
}

