extern crate sha1;

use sha1::{Digest, Sha1};

pub const HASH_SIZE: usize = 160;
pub const HASH_SIZE_BYTES: usize = HASH_SIZE / 8;

pub struct KeyHash {
    raw: Digest
}

impl KeyHash {
    pub fn from_string(data: &str) -> KeyHash {
        let mut m = Sha1::new();
        m.update(data.as_bytes());
        KeyHash { raw: m.digest() }
    }

    pub fn from_bytes(data: &[u8; HASH_SIZE_BYTES]) -> KeyHash {
        let mut m = Sha1::new();
        m.update(data);
        KeyHash { raw: m.digest() }
    }

    pub fn as_string(&self) -> String {
        self.raw.to_string()
    }

    pub fn as_bytes(&self) -> [u8; HASH_SIZE_BYTES] {
        self.raw.bytes()
    }
}