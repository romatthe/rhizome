use arrayvec::ArrayVec;
use hash::HASH_SIZE;
use node::NodeId;

mod bucket;
mod contact;
pub use self::bucket::Bucket;
pub use self::contact::Contact;

pub const MAX_BUCKET_SIZE: u8 = 20;

pub struct RoutingTable {
    buckets: ArrayVec<[Bucket; HASH_SIZE]>
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

    pub fn update(&self, node: NodeId) {

    }
}