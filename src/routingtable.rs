use arrayvec::ArrayVec;
use hash::HASH_SIZE;
use node::{NodeId};

pub struct RoutingTable {
    buckets: ArrayVec<[Bucket; HASH_SIZE]>
}

#[derive(Debug)]
pub struct Bucket {
    pub nodes: Vec<NodeId>
}

impl RoutingTable {
    pub fn new() -> RoutingTable {
        let mut new_buckets = ArrayVec::<[Bucket; HASH_SIZE]>::new();
        for _ in 0..HASH_SIZE {
            new_buckets.push(Bucket::new());
        }
        RoutingTable { buckets: new_buckets }
    }

    pub fn bucket_at(&self, index: usize) -> &Bucket {
        &self.buckets.as_slice()[index]
    }
}

impl Bucket {
    pub fn new() -> Bucket {
        Bucket { nodes: vec![] }
    }
}