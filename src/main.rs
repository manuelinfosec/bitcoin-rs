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

    // perform pattern matching on the module for routin
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
            printlnl1!("Node: Register a node on the network and list all nodes on the network")
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
