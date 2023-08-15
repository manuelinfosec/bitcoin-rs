use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Blockchain {
    pub index: u32,
    pub timestamp: u64,
    pub tx: Vec<String>,
    pub previous_block: String,
    pub nonce: u32,
    pub hash: String,
}

impl Blockchain {
    pub fn default() -> Blockchain {
        Blockchain {
            index: 0,
            timestamp: 0,
            tx: vec![],
            previous_block: "".to_string(),
            nonce: 0,
            hash: "".to_string(),
        }
    }
}
