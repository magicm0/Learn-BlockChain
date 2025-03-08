use crate::serializer::*;

use super::block::*;

const PRE_HASH: &str = "Bitcoin hit $$60000";

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> Self {
        BlockChain {
            blocks: vec![Self::genesis_block()],
        }
    }

    fn genesis_block() -> Block {
        let ser = serialize(PRE_HASH);
        let hash = hash_str(&ser);
        Block::new("创世区块".to_string(), hash)
    }

    pub fn add_block(&mut self, data: String) {
        let pre_block = &self.blocks[self.blocks.len() - 1];
        let pre_hash = pre_block.hash.clone();
        let new_block = Block::new(data, pre_hash);
        self.blocks.push(new_block);
    }

    pub fn block_info(&self) {
        for b in self.blocks.iter() {
            println!("{:#?}", b)
        }
    }
}
