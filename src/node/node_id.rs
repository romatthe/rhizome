extern crate rand;

use std::fmt;
use std::io::Cursor;
use std::ops::BitXor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use hash::HASH_SIZE_BYTES;
use rand::Rng;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct NodeId {
    pub value: Vec<u8>
}

pub trait IntoBytesVec {
    fn into_bytes_vec(self) -> Vec<u8>;
}

impl NodeId {
    pub fn new() -> NodeId {
        let mut rng = rand::thread_rng();
        let id = rng.gen_iter::<u8>().take(HASH_SIZE_BYTES).collect::<Vec<u8>>();

        NodeId { value: id }
    }

    pub fn from_bytes<I>(data: I) -> NodeId where I: IntoBytesVec {
        NodeId { value: data.into_bytes_vec() }
    }

    pub fn count_leading_zeroes(&self) -> u32 {
        let result = self.value.iter().find(|&byte| *byte != 0b00);
        match result {
            Some(byte)  => byte.leading_zeros(),
            None        => 0
        }
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut rdr = Cursor::new(&self.value);
        write!(f, "{:?}", rdr.read_u64::<BigEndian>().unwrap())
    }
}

impl fmt::Binary for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let fmt = &self.value.iter().fold(String::from(""), |acc, &byte| format!("{}{:08b}", &acc, &byte));
        write!(f, "{}", fmt)
    }
}

impl IntoBytesVec for [u8; 20] {
    fn into_bytes_vec(self) -> Vec<u8> {
        self.to_vec()
    }
}

impl IntoBytesVec for Vec<u8> {
    fn into_bytes_vec(self) -> Vec<u8> {
        self
    }
}

impl <'a> BitXor<&'a NodeId> for &'a NodeId {
    type Output = NodeId;

    fn bitxor (self, rhs: Self) -> NodeId {
        let dist = self.value.iter()
            .zip(rhs.value.iter())
            .map(|(byte1, byte2)| byte1 ^ byte2)
            .collect::<Vec<u8>>();

        NodeId::from_bytes(dist)
    }
}