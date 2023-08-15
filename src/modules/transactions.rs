use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub timestamp: u64,
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vin {
    pub sender: String,
    pub amount: u32,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vout {
    pub receiver: String,
    pub amount: u32,
    pub hash: String,
}

impl Transaction {
    pub fn default() -> Transaction {
        Transaction {
            timestamp: 0,
            vin: vec![],
            vout: vec![],
            hash: "".to_string(),
        }
    }
}
