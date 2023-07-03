// interacting with the system environment
use std::env;
// interacting with current process
use std::process;

// structs for routing to various components based on the module argument
use routes::{Account, Blockchain, Miner, Node, Transaction};

mod routes;

// TODO: Create `usage()` function for displaying help message

fn main() {
    // Collect command-line arguments to a vector
    let argv: Vec<String> = env::args().collect();

    // check for insufficent number of arguments passed
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
                    "get" => Account::get(),

                    // create an account (consisting of private, public keys and addresses) to..
                    // ... the local database
                    "create" => Account::create(argv),

                    // get the current account from the local database
                    "current" => Account::current(),

                    // handle for invalid method argument
                    _ => eprintln!("Account: Invalid account method")
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments
                eprintln!("bitcoin-rs: Account requires a method")
            }
        }
        "tx" => {
            // call the tx module with the rest of argv as arguments
            println!("Transactions: creating and listing all transactions on the blockchain");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // transfer bitcoin from one node to another
                    "transfer" => Transaction::transfer(argv),

                    // list all transactions on the blockchain
                    "list" => Transaction::list(),

                    _ => eprintln!("Transactions: \"{method}\" is not a {module} module")
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments
                eprintln!("bitcoin-rs: Transactions requires a method")
            }
        }
        "blockchain" => {
            // call the blockchain module with the rest of argv as arguments
            println!("Blockchain: listing all blocks on the blockchain");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // list all blocks on the blockchain
                    "list" => Blockchain::list(),

                    // handle for invalid method
                    _ => eprintln!("Blockchain: \"{method}\" is not a {module} module")
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments
                eprintln!("bitcoin-rs: Blockchain requires a method")
            }
        }
        "miner" => {
            // call the miner module with the rest of argv as arguments
            println!("Miner: start a node and mine transactions");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // start a miner on the network
                    "start" => Miner::start(argv),

                    // handle for invalid method
                    _ => eprintln!("Miner: \"{method}\" is not a {module} module")
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments
                eprintln!("bitcoin-rs: Miner requires a method")
            }
        }
        "node" => {
            // call the node module with the rest of argv as arguments
            println!("Node: Register a node on the network and list all nodes on the network");

            // check if an argument exists for methods
            if let Some(method) = argv.get(2) {
                match &method[..] {
                    // register a node locally and on the network
                    "add" => Node::add(argv),

                    // list all the nodes on the network
                    "list" => Node::list(),

                    // handle for invalid method
                    _ => eprintln!("Node: \"{method}\" is not a {module} module")
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments
                eprintln!("bitcoin-rs: Node requires a method")
            }
        }
        // matching for wildcard (important when using match for `&str`)
        _ => {
            // throw invalid module name
            eprintln!("Invalid module name");

            // throw help message
        }
    }
}