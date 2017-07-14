extern crate rand;
extern crate sha1;

mod node;

use node::Node;

fn main() {
    let poemel = (1..20).zip((21..40)).map(|(a, b)| a ^ b).collect::<Vec<i32>>();
    println!("Zip example {:?}", poemel);
    let new_node = Node::new();
    println!("New Node id acquired: {}", new_node.id);
    let new_node2 = Node::new();
    println!("New Node id acquired: {}", new_node2.id);
    println!("Distance betweeen two nodes: {}", new_node.distance(new_node2));
}

