// interacting with the system environment
use std::env;
// interacting with current process
use std::process;

// structs for routing to various components based on the module argument
use routes::{AccountRoute, BlockchainRoute, MinerRoute, NodeRoute, TransactionRoute};

mod modules {
    pub mod account;
    pub mod blockchain;
    pub mod generics;
    pub mod miner;
    pub mod node;
    pub mod transactions;
}

mod database;
mod p2p;
mod routes;

#[tokio::main]
async fn main() {
    // Collect command-line arguments to a vector
    let argv: Vec<String> = env::args().collect();

    // check for insufficient number of arguments passed
    if argv.len() == 1 {
        // throw help message
        eprintln!("Incorrect number of arguments");

        // exit the program
        process::exit(0);
    }

    // collect the current module from arguments
    let module: &str = &argv[1];

    // perform pattern matching on the module for routing
    match module {
        "help" => {
            // throw help message
            println!("Help message comes here!");

            // exit the program
            process::exit(0);
        }
        "account" => {
            // call the account module with the rest of argv as arguments
            println!("Account: creating and listing available accounts");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                // perform pattern matching for methods
                // value (for match) is the sliced version of `String` which is `&str`
                match &method[..] {
                    // get all accounts from the local database
                    "get" => AccountRoute::get(),

                    // create an account (consisting of private, public keys and addresses) to..
                    // ... the local database
                    "create" => AccountRoute::create(argv),

                    // get the current account from the local database
                    "current" => AccountRoute::current(),

                    // handle for invalid method argument
                    _ => eprintln!("Account: Invalid account method."),
                }
            }
            // in the case where no arguments exists
            // the `None` variant of the `Option` is handled
            else {
                // throw error for invalid number of arguments
                eprintln!("bitcoin-rs: Account requires a method.")
            }
        }
        "tx" => {
            // call the tx module with the rest of argv as arguments
            println!("Transactions: creating and listing all transactions on the blockchain");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // transfer bitcoin from one node to another
                    "transfer" => TransactionRoute::transfer(argv),

                    // list all transactions on the blockchain
                    "list" => TransactionRoute::list(),

                    _ => eprintln!("Transactions: \"{method}\" is not a {module} module"),
                }
            }
            // in the case where no arguments exists
            // the `None` variant of the `Option` is handled
            else {
                // throw error for invalid number of arguments
                eprintln!("bitcoin-rs: Transactions requires a method.")
            }
        }
        "blockchain" => {
            // call the blockchain module with the rest of argv as arguments
            println!("Blockchain: listing all blocks on the blockchain");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // list all blocks on the blockchain
                    "list" => BlockchainRoute::list(),

                    // handle for invalid method
                    _ => eprintln!("Blockchain: \"{method}\" is not a {module} module"),
                }
            }
            // in the case where no arguments exists
            // the `None` variant of the `Option` is handled
            else {
                // throw error for invalid number of arguments
                eprintln!("bitcoin-rs: Blockchain requires a method.")
            }
        }
        "miner" => {
            // call the miner module with the rest of argv as arguments
            println!("Miner: start a node and mine transactions");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // start a miner on the network
                    "start" => MinerRoute::start(argv),

                    // handle for invalid method
                    _ => eprintln!("Miner: \"{method}\" is not a {module} module"),
                }
            }
            // in the case where no arguments exists
            // the `None` variant of the `Option` is handled
            else {
                // throw error for invalid number of arguments
                eprintln!("bitcoin-rs: Miner requires a method.")
            }
        }
        "node" => {
            // call the node module with the rest of argv as arguments

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // register a node locally and on the network
                    "add" => NodeRoute::add(argv),

                    // list all the nodes on the network
                    "list" => NodeRoute::list(),

                    // start a node locally
                    "start" => NodeRoute::start(argv).await,

                    // handle for invalid method
                    _ => eprintln!("Node: \"{method}\" is not a {module} module"),
                }
            }
            // in the case where no arguments exists
            // the `None` variant of the `Option` is handled
            else {
                // throw error for invalid number of arguments
                eprintln!("bitcoin-rs: Node requires a method.")
            }
        }
        // matching for wildcard (important when using match for `&str`)
        _ => {
            // throw invalid module name
            eprintln!("bitcoin-rs: \"{module}\" is not a valid module.");

            // throw help message
        }
    }
}

// TODO: Create `usage()` function for displaying help message

// fn main() {
//     // let mut node_db = database::NodeDB::new();
//     // let mut blockchain = database::BlockchainDB::new();
//     // let mut values: Vec<String> = blockchain.read();
//     // values.push(String::from("testing"));
//     // let block = BChain {
//     //     index: 1,
//     //     timestamp: 1625741692,
//     //     tx: vec![String::from("86dc2f1acfd239004b8a7b515241070204d5da0ccebf82140416623d3380766d")],
//     //     previous_block: "".to_string(),
//     //     nonce: 744,
//     //     hash: "00000fee282315563171dccc13a2eab380ee82351e7efa6ab13249754f283758".to_string(),
//     // };
//     // println!("{block:?}");
//     // blockchain.write(block).unwrap();
//     // // node_db.write(String::from("testing")).expect("Could not write");
//     // let values: Vec<BChain> = blockchain.read();
//     let txn_db: TransactionDB = TransactionDB::new();
//
//     let transaction = Txn {
//         timestamp: 1528972068,
//         vin: Vec::new(),
//         vout: vec![modules::transactions::Vout {
//             receiver: "1L8Q3xJyk5MnWoV1Qz6sfT57yGB6bA7DgR".to_string(),
//             amount: 20,
//             hash: "1962d0500de06d90e74249659d12640b32eafbe1ba02fea578637f25464eb220".to_string(),
//         }],
//         hash: "86dc2f1acfd239004b8a7b515241070204d5da0ccebf82140416623d3380766d".to_string(),
//     };
//
//     txn_db.insert(transaction).unwrap();
//     let values: Vec<Txn> = txn_db.read();
//
//     println!("{values:?}");
// }
