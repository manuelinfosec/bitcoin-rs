use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Write};

use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use serde_json;

// use crate::modules::node::Nodes;

const BASEDBPATH: &str = "data";
const NODEFILE: &str = "nodes.json";
const TXFILE: &str = "txn.json";
const UNTXFILE: &str = "untxn.json";
const ACCOUNTDB: &str = "accounts.json";
const BLOCKCHAINDB: &str = "blockchain.json";

pub trait BaseDB {
    fn read<T: DeserializeOwned>(&self) -> Vec<T>;
    fn write<T: Serialize + DeserializeOwned>(&self, data: T) -> io::Result<()>;
    fn clear(&self) -> io::Result<()>;
    fn find_all(&self) -> Vec<&str>;
    fn insert(&self, item: String) -> io::Result<()>;
    fn hash_insert(&self, item: String) -> io::Result<()>;
}

// Nodes in the network
pub struct NodeDB {
    file_path: String, // database location
}

// Local user accounts
pub struct AccountDB {
    file_path: String, // database location
}

// Blocks in the database
pub struct BlockchainDB {
    file_path: String,  // database location
}

// Transactions in the database
pub struct TransactionDB {
    file_path: String,  // database location
}

// Transactions that don't store in the database
pub struct UnTransactionDB {
    file_path: String,  // database location
}

// Native methods for the Nodes database
impl NodeDB {
    // create an instance of the Nodes database
    pub fn new() -> NodeDB {
        // perform initialize with database location
        NodeDB {
            file_path: String::from(format!("/{BASEDBPATH}/{NODEFILE}")) // set a default empty string
        }
    }
}

// Native methods for the Blockchain database
impl BlockchainDB {
    // create an instance of the Blockchain database
    pub fn new() -> BlockchainDB {
        // perform initialization with the database location
        BlockchainDB {
            file_path: String::from(format!("/{BASEDBPATH}/{BLOCKCHAINDB}"))
        }
    }

    #[allow(unused_variables)]
    fn find(&self, hash: String) -> HashMap<String, String> {
        HashMap::new()
    }

    fn insert(&self) -> io::Result<()> {
        self.hash_insert("Test".to_string())
    }
}

// Native methods for the accounts database
impl AccountDB {
    // create an instance of the Account database
    pub fn new() -> AccountDB {
        // perform initiallization with the database locatin
        AccountDB {
            file_path: String::from(format!("/{BASEDBPATH}/{ACCOUNTDB}"))
        }
    }
    fn find_one(&self) -> &str {
        ""
    }
}


// Inherited methods from BaseDB trait
impl BaseDB for NodeDB {
    // read the database return an object that is deserializable
    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(e) => {
                return Vec::new();
            }
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return Vec::new();
        }

        // deserialize from string to Vec<Node>
        let data: Result<Vec<T>, serde_json::Error> = serde_json::from_str(&raw);

        // check for deserialization errors
        match data {
            Ok(data) => {   // return the deserialized data
                data
            }
            Err(err) => {   // return a new vector as form of error handling
                Vec::new()
            }
        }
    }

    // write an item to the database | accepting parameters that can be serialized or deserialized
    fn write<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        // read the entire database to vector buffer (there should be a better to do this)
        let mut data: Vec<T> = self.read();

        // push item to the buffer
        data.push(item);

        let json_data = serde_json::to_string(&data)?;      // serialize buffer vector to string
        let mut file = File::create(&self.file_path)?;             // overwrite existing database
        file.write_all(json_data.as_bytes())?;                     // write serialized string to the database

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


impl BaseDB for AccountDB {
    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(_) => return Vec::new()
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return Vec::new();
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

    // write an item to the database | accepting parameters that can be serialized or deserialized
    fn write<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        // read the entire database to vector buffer (there should be a better to do this)
        let mut data: Vec<T> = self.read();

        // push item to the buffer
        data.push(item);

        let json_data = serde_json::to_string(&data)?;      // serialize buffer vector to string
        let mut file = File::create(&self.file_path)?;             // overwrite existing database
        file.write_all(json_data.as_bytes())?;                     // write serialized string to the database

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


impl BaseDB for BlockchainDB {
    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(&self.file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(_) => return Vec::new()
        };

        // handle errors when reading the file
        if let Err(_) = file.read_to_string(&mut raw) {
            // return an empty vector
            return Vec::new();
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

    // write an item to the database | accepting parameters that can be serialized or deserialized
    fn write<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        // read the entire database to vector buffer (there should be a better to do this)
        let mut data: Vec<T> = self.read();

        // push item to the buffer
        data.push(item);

        let json_data = serde_json::to_string(&data)?;      // serialize buffer vector to string
        let mut file = File::create(&self.file_path)?;             // overwrite existing database
        file.write_all(json_data.as_bytes())?;                     // write serialized string to the database

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
