// interacting with the system environment
use std::env;
// interacting with current process
use std::process;

fn main() {
    // define the allowed modules
    const MODULES: [&str; 5] = ["account", "tx", "blockchain", "miner", "node"];

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
    let module: &String = &argv[1];

    // compare current module with permitted modules
    if !MODULES.iter().any(|e| e == module) {
        // throw error message
        eprintln!("bitcoin-rs: {} is not a module.", module);

        // exit the program
        process::exit(0);
    }
}
