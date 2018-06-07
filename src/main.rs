extern crate sha2;

mod block;
mod blockchain;
mod utils;

use block::Block;
use blockchain::Blockchain;

fn main() {
    let block = Block::genesis();
    println!("{}", block);
}
