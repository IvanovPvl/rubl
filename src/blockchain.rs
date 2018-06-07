use block::Block;

pub struct Blockchain {
    blocks: Vec<Block>,
}

impl Blockchain {
    pub fn new() -> Blockchain {
        Blockchain {
            blocks: vec![Block::genesis()],
        }
    }

    pub fn add_block(&mut self, data: Vec<u8>) {
        let new_block: Block;
        {
            let prev_block: &Block = self.blocks.last().unwrap();
            new_block = Block::new(data, prev_block.prev_block_hash.clone()); // TODO: clone
        }
        self.blocks.push(new_block);
    }
}
