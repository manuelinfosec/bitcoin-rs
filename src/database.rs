use std::collections::HashMap;
use std::io;

use crate::routes::Account;

const BASEDBPATH: &str = "data";
const NODEFILE: &str = "node";
const TXFILE: &str = "txn";
const UNTXFILE: &str = "untxn";
const ACCOUNTSDB: &str = "accounts";
const BLOCKCHAINDB: &str = "blockchain";

trait BaseDB {
    fn set_path(&mut self);
    fn read(&self) -> Vec<&str>;
    fn write(&self, item: String) -> io::Result<()>;
    fn clear(&self) -> io::Result<()>;
    fn find_all(&self) -> Vec<&str>;
    fn insert(&self, item: String) -> io::Result<()>;
    fn hash_insert(&self, item: String) -> io::Result<()>;
}

struct NodeDB {
    file_path: String,
}

impl BaseDB for NodeDB {
    fn set_path(&mut self) {
        self.file_path = String::from(NODEFILE);
    }

    fn read(&self) -> Vec<&str> {
        todo!()
    }

    fn write(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn clear(&self) -> io::Result<()> {
        todo!()
    }

    fn find_all(&self) -> Vec<&str> {
        todo!()
    }

    fn insert(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn hash_insert(&self, item: String) -> io::Result<()> {
        todo!()
    }
}

struct AccountDB {
    file_path: String,
}

impl BaseDB for AccountDB {
    fn set_path(&mut self) {
        todo!()
    }

    fn read(&self) -> Vec<&str> {
        todo!()
    }

    fn write(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn clear(&self) -> io::Result<()> {
        todo!()
    }

    fn find_all(&self) -> Vec<&str> {
        todo!()
    }

    fn insert(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn hash_insert(&self, item: String) -> io::Result<()> {
        todo!()
    }
}

impl AccountDB {
    fn find_one(&self) -> &str {
        ""
    }
}

struct BlockchainDB {
    file_path: String,
}

impl BaseDB for BlockchainDB {
    fn set_path(&mut self) {
        todo!()
    }

    fn read(&self) -> Vec<&str> {
        todo!()
    }

    fn write(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn clear(&self) -> io::Result<()> {
        todo!()
    }

    fn find_all(&self) -> Vec<&str> {
        todo!()
    }

    fn insert(&self, item: String) -> io::Result<()> {
        todo!()
    }

    fn hash_insert(&self, item: String) -> io::Result<()> {
        todo!()
    }
}

impl BlockchainDB {
    #[allow(unused_variables)]
    fn find(&self, hash: String) -> HashMap<String, String> {
        HashMap::new()
    }

    fn insert(&self) -> io::Result<()> {
        self.hash_insert()
    }
}