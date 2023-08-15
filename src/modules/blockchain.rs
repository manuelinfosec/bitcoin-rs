use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub index: u32,
    pub timestamp: u64,
    pub tx: Vec<String>,
    pub previous_block: String,
    pub nonce: u32,
    pub hash: String,
}

impl Block {
    pub fn default() -> Block {
        Block {
            index: 0,
            timestamp: 0,
            tx: vec![],
            previous_block: "".to_string(),
            nonce: 0,
            hash: "".to_string(),
        }
    }
}
