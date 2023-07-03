// interacting with the system environment
use std::env;
// interacting with current process
use std::process;

// structs for routing to various components based on the module argument
use routes::{Account, Blockchain, Miner, Node, Transaction};

mod routes;

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
                    "get" => {
                        // get all accounts from the local database
                        Account::get();
                    }
                    "create" => {
                        // create an account (consisting of private, public keys and addresses) to..
                        // ... the local database
                        Account::create(argv);
                    }
                    "current" => {
                        // get the current account from the local database
                        Account::current();
                    }
                    // handle for wildcard
                    _ => {
                        // throw error for invalid account method
                        eprintln!("Account: Invalid account method");
                    }
                }
            }
            // in the case where no arguments exists
            // the `None` varient of the `Option` is handled
            else {
                // throw errow for invalid number of arguments for the account module
                eprintln!("bitcoin-rs: Account requires a method")
            }
        }
        "tx" => {
            // call the tx module with the rest of argv as arguments
        }
        "blockchain" => {
            // call the blockchain module with the rest of argv as arguments
            println!("Blockchain: listing all blocks on the blockchain");
        }
        "miner" => {
            // call the miner module with the rest of argv as arguments
            println!("Miner: start a node and mine transactions");
        }
        "node" => {
            // call the node module with the rest of argv as arguments
            println!("Node: Register a node on the network and list all nodes on the network")
        }
        // matching for wildcard (important when using match for `&str`)
        _ => {
            // throw invalid module name
            eprintln!("Invalid module name");

            // throw help message
        }
    }

    // TODO:
    // Create a HashMap of modules and their methods as values in a vector
    // Query the map with the supplied module and check against the vector.
    // if the first value of the argument (the method) exists in the vector.
}
