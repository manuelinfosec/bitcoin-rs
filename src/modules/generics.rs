use super::blockchain::Blockchain;
use super::transactions::Transaction;

pub trait HasHashField {
    fn hash(&self) -> String;
}

impl HasHashField for Transaction {
    fn hash(&self) -> String {
        self.hash.to_string()
    }
}

impl HasHashField for Blockchain {
    fn hash(&self) -> String {
        self.hash.to_string()
    }
}
