use super::serializer::*;
use bincode::Encode;
use chrono::prelude::*;
use serde::Serialize;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Debug, PartialEq, Eq, Encode)]
pub struct BlockHeader {
    pub time: i64,
    pub pre_hash: String,
    pub txs_hash: String,
}

#[derive(Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub tranxs: String,
    pub hash: String,
}

impl Block {
    pub fn new(txs: String, pre_hash: String) -> Self {
        println!("Start mining ...");
        thread::sleep(Duration::from_secs(3));

        let time = Utc::now().timestamp();
        let txs_ser = serialize(&txs);
        let txs_hash = hash_str(&txs_ser);
        let mut block = Block {
            header: BlockHeader {
                time,
                pre_hash,
                txs_hash,
            },
            tranxs: txs,
            hash: "".to_string(),
        };
        block.set_hash();
        println!("Produce a new block!\n");
        block
    }

    fn set_hash(&mut self) {
        let header = serialize(&(self.header));
        self.hash = hash_str(&header);
    }
}
