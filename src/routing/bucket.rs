use node::NodeId;

#[derive(Debug)]
pub struct Bucket {
    pub nodes: Vec<NodeId>
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket { nodes: vec![] }
    }
}