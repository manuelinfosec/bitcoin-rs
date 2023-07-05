use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Accounts {
    pub pubkey: String,
    pub address: String,
}