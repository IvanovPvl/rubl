use sha2::{Digest, Sha256};
use std;
use std::fmt::{Display, Formatter};

use utils;

pub struct Block {
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub prev_block_hash: Vec<u8>,
    pub hash: Vec<u8>,
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(
            f,
            "(Prev. hash: {}\nHash: {})",
            utils::to_hex(&self.prev_block_hash),
            utils::to_hex(&self.hash)
        )
    }
}

impl Block {
    pub fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = utils::timestamp_millis();
        let hash = Vec::new();
        let mut block = Block {
            timestamp,
            data,
            prev_block_hash,
            hash,
        };
        block.set_hash();
        block
    }

    pub fn genesis() -> Block {
        Block::new("Genesis Block".as_bytes().to_vec(), Vec::new())
    }

    fn set_hash(&mut self) {
        let timestamp_str = self.timestamp.to_string();
        let concat = [
            &self.prev_block_hash[..],
            &self.data[..],
            timestamp_str.as_bytes(),
        ].concat();
        let headers = concat.as_slice();
        let digest = Sha256::digest(headers);
        self.hash = digest.to_vec();
    }
}
