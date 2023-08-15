use super::blockchain::Block;
use super::transactions::Transaction;

pub trait HasHashField {
    fn hash(&self) -> String;
}

impl HasHashField for Transaction {
    fn hash(&self) -> String {
        self.hash.to_string()
    }
}

impl HasHashField for Block {
    fn hash(&self) -> String {
        self.hash.to_string()
    }
}
