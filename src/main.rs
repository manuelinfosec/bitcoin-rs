// interacting with the system environment
use std::env;

fn main() {
    // Collect command-line arguments to a vector
    let env: Vec<String> = env::args().collect();
}
