use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;

// use crate::modules::node::Nodes;

const BASEDBPATH: &str = "data";
const NODEFILE: &str = "node";
const TXFILE: &str = "txn";
const UNTXFILE: &str = "untxn";
const ACCOUNTSDB: &str = "accounts";
const BLOCKCHAINDB: &str = "blockchain";

trait BaseDB {
    fn set_path(&mut self);
    fn read<T: DeserializeOwned>(&self) -> Vec<T>;
    fn write<T: serde::Serialize>(&self, data: &T) -> io::Result<()>;
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

    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(_) => return vec![]
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return vec![];
        }


        // deserialize from string to Vec<Node>
        let data: Result<Vec<T>, serde_json::Error> = serde_json::from_str(&raw);

        // check for deserialization errors
        match data {
            // return the deserialized data
            Ok(data) => data,
            // return a new vector as form of error handling
            Err(_) => Vec::new(),
        }
    }

    fn write<T: serde::Serialize>(&self, data: &T) -> io::Result<()> {
        let json_data = serde_json::to_string(data)?;

        Ok(())
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
        Ok(())
    }
}

struct AccountDB {
    file_path: String,
}

impl BaseDB for AccountDB {
    fn set_path(&mut self) {
        todo!()
    }

    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(_) => return vec![]
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return vec![];
        }

        // deserialize from string to Vec<Node>
        let data: Result<Vec<T>, serde_json::Error> = serde_json::from_str(&raw);

        // check for deserialization errors
        match data {
            // return the deserialized data
            Ok(data) => data,
            // return a new vector as form of error handling
            Err(_) => Vec::new(),
        }
    }


    fn write<T: serde::Serialize>(&self, data: &T) -> io::Result<()> {
        let json_data = serde_json::to_string(data);
        Ok(())
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

    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        todo!()
    }

    fn write<T: serde::Serialize>(&self, data: &T) -> io::Result<()> {
        let json_data = serde_json::to_string(data);
        Ok(())
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
        self.hash_insert("Test".to_string())
    }
}