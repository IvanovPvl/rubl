extern crate sha2;

use sha2::{Digest, Sha256};
use std::time::{SystemTime, UNIX_EPOCH};

fn timestamp_millis() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .unwrap();
    since_the_epoch.as_secs() * 1000 + since_the_epoch.subsec_nanos() as u64 / 1_000_000
}

struct Block {
    timestamp: u64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    fn new(data: Vec<u8>, prev_block_hash: Vec<u8>) -> Block {
        let timestamp = timestamp_millis();
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

    fn genesis() -> Block {
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

struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    fn new() -> Blockchain {
        Blockchain { blocks: vec![Block::genesis()] }
    }

    fn add_block(&mut self, data: Vec<u8>) {
        let new_block: Block;
        {
            let prev_block: &Block = self.blocks.last().unwrap();
            new_block = Block::new(data, prev_block.prev_block_hash.clone()); // TODO: clone
        }
        self.blocks.push(new_block);
    }
}

fn main() {
    let mut block = Block {
        data: "data".as_bytes().to_vec(),
        hash: "hash".as_bytes().to_vec(),
        prev_block_hash: "prev".as_bytes().to_vec(),
        timestamp: 123,
    };

    block.set_hash();

    let s = "123";
    println!("{:?}", s.as_bytes());
}
