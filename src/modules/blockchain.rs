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