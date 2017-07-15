use std::iter;
use node::{NodeId, HASH_SIZE};

pub struct RoutingTable {
    pub buckets: [Vec<NodeId>; HASH_SIZE]
}

impl RoutingTable {
    pub fn new() -> RoutingTable {
        let mut new_buckets: Vec<Vec<NodeId>> = Vec::with_capacity(HASH_SIZE);
        new_buckets.iter().map(|vec| vec![]).collect();

        RoutingTable { buckets: new_buckets.as_ref() }
    }
}