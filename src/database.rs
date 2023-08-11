use std::fs::File;
use std::io::{self, Read, Write};

use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json;

use modules::account::Account;
use modules::blockchain::Blockchain;

use crate::modules;
use crate::modules::generics::HasHashField;
use crate::modules::transactions::Transaction;

const BASEDBPATH: &str = "data";
const NODEFILE: &str = "nodes.json";
const TXFILE: &str = "txn.json";
const UNTXFILE: &str = "untxn.json";
const ACCOUNTDB: &str = "accounts.json";
const BLOCKCHAINDB: &str = "blockchain.json";

pub trait BaseDB {
    // get current path to local database
    fn get_path(&self) -> String;

    // read the database || return an object that is deserializable
    fn read<T: DeserializeOwned>(&self) -> Vec<T> {
        let file_path: String = self.get_path();
        // create an empty string to save data read from file
        let mut raw: String = String::new();

        // check if the file exists or return the file for reading
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            // handle file open error, by returning an empty vector
            Err(_) => {
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
            Ok(data) => {
                // return the deserialized data
                data
            }
            Err(_) => {
                // return a new vector as form of error handling
                Vec::new()
            }
        }
    }

    // write an item to the database | accepting parameters that can be serialized or deserialized
    fn write<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        // get current path to local database
        let file_path: String = self.get_path();

        // read the entire database to vector buffer (there should be a better to do this)
        let mut data: Vec<T> = self.read();
        // println!("{data:?}");

        // push item to the buffer
        data.push(item);

        let json_data = serde_json::to_string(&data)?; // serialize buffer vector to string
        let mut file = File::create(file_path)?; // overwrite existing database
        file.write_all(json_data.as_bytes())?; // write serialized string to the database

        Ok(())
    }

    // erase the database
    fn clear(&self) -> io::Result<()> {
        let file_path = self.get_path();
        // truncate the database file
        File::create(file_path)?;
        Ok(())
    }

    // return all objects in the local database
    fn find_all<T: DeserializeOwned>(&self) -> Vec<T> {
        // read the local database
        self.read()
    }

    // write an item to the local database
    fn insert<T: Serialize + DeserializeOwned>(&self, item: T) -> io::Result<()> {
        self.write(item)
    }

    // insert a transaction hash if it doesn't exist in the local database
    fn hash_insert<T>(&self, item: T) -> io::Result<()>
    where
        T: Serialize + DeserializeOwned + HasHashField,
    {
        // flag for checking if a hash already exists
        let mut exists = false;

        // loop through all available hashes
        for obj in self.find_all::<T>() {
            // compare the hash value of the item to be inserted with the hash value of blocks in...
            // ..the database. If they are equal, an object with the same hash value already exists.
            if item.hash() == obj.hash() {
                // set the flag to indicate that a matching hash was found
                exists = true;
                // exit the loop early since a match was found
                break;
            }
        }

        if !exists {
            // If the flag has not been updated (no matching hash was found),
            // write the item to database using `self.write()`.
            self.write(item)?;
        }

        // return a successful I/O result
        Ok(())
    }
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
    file_path: String, // database location
}

// Transactions in the database
pub struct TransactionDB {
    file_path: String, // database location
}

// Un-mined Transactions
pub struct UnTransactionDB {
    file_path: String, // database location
}

// Native methods for the Nodes database
impl NodeDB {
    // create an instance of the Nodes database
    pub fn new() -> NodeDB {
        // perform initialize with database location
        NodeDB {
            file_path: String::from(format!("{BASEDBPATH}/{NODEFILE}")), // set a default empty string
        }
    }
}

// Native methods for the Blockchain database
impl BlockchainDB {
    // create an instance of the Blockchain database
    pub fn new() -> BlockchainDB {
        // perform initialization with the database location
        BlockchainDB {
            file_path: String::from(format!("{BASEDBPATH}/{BLOCKCHAINDB}")),
        }
    }

    fn find(&self, hash: String) -> Blockchain {
        // initialize a default `Blockchain` with empty values
        let mut default: Blockchain = Blockchain::default();

        // iterate over all objects for type `Blockchain` from the local database
        for item in self.find_all::<Blockchain>() {
            // check if hash of the current item matches the provided hash
            if item.hash == hash {
                // if a match is found, update the current item to the default item.
                default = item;
            }
        }
        // return the final value of the `default` variable
        default
    }

    // insert a record to the blockchain
    fn insert(&self, item: Blockchain) -> io::Result<()> {
        // insert item by hash or do nothing if hash already exists
        self.hash_insert(item)
    }
}

// Native methods for the accounts database
impl AccountDB {
    // create an instance of the Account database
    pub fn new() -> AccountDB {
        // perform initialization with the database location
        AccountDB {
            file_path: String::from(format!("{BASEDBPATH}/{ACCOUNTDB}")),
        }
    }

    // get the last account from the database
    fn find_one(&self) -> Option<Account> {
        // read for all accounts from the local database
        let accounts: Vec<Account> = self.read();

        // get a copied value of the last account
        accounts.get(0).cloned()
    }
}

impl TransactionDB {
    // create an instance of the Transaction database
    pub fn new() -> TransactionDB {
        // perform initialization with the database location
        TransactionDB {
            file_path: String::from(format!("{BASEDBPATH}/{TXFILE}")),
        }
    }

    fn find(&self, hash: String) -> Transaction {
        // initialize a default `Transaction` with empty values
        let mut default: Transaction = Transaction::default();

        // iterate over all objects for type `Transaction` from the local database
        for item in self.find_all::<Transaction>() {
            // check if hash of the current item matches the provided hash
            if item.hash == hash {
                // if a match is found, update the current item to the default item.
                default = item;
            }
        }

        // return the final value of the `default` variable
        default
    }

    // Insert a single transaction (implementing it as an iterator) or multiple transactions
    // this can also be achieved with method overloading
    pub fn insert(&self, txn: Transaction) -> io::Result<()> {
        // iterate over each items in the transaction parameter.
        // this works because `transaction` implements the `IntoIterator` trait
        // for txn in transaction {
        // insert the transaction to the local database by its hash
        // or do nothing if the hash already exists
        self.hash_insert(txn)?;
        // }

        // return a successful I/O result
        Ok(())
    }
}

// Native methods for un-mined transactions
impl UnTransactionDB {
    // create an instance of the UnTransaction database
    pub fn new() -> UnTransactionDB {
        // perform initialization with the database location
        UnTransactionDB {
            file_path: String::from(format!("{BASEDBPATH}/{UNTXFILE}")),
        }
    }

    fn all_hashes(&self) -> Vec<String> {
        let mut hashes: Vec<String> = Vec::new();

        for item in self.find_all::<Transaction>() {
            hashes.push(item.hash)
        }

        hashes
    }
}

// Inherited methods from BaseDB trait
impl BaseDB for NodeDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_string()
    }
}

impl BaseDB for AccountDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_string()
    }
}

impl BaseDB for BlockchainDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_string()
    }
}

impl BaseDB for TransactionDB {
    // get current path to local database
    fn get_path(&self) -> String {
        self.file_path.to_string()
    }
}

impl BaseDB for UnTransactionDB {
    fn get_path(&self) -> String {
        self.file_path.to_string()
    }
}
