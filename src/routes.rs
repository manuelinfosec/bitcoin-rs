pub struct AccountRoute {}

pub struct BlockchainRoute {}

pub struct MinerRoute {}

pub struct NodeRoute {}

pub struct TransactionRoute {}

impl NodeRoute {
    #[allow(unused_variables)]
    pub fn add(args: Vec<String>) {
        // add the node locally

        // broadcast all local nodes
    }

    pub fn list() {
        // query all local nodes to vector

        println!("Printing local nodes...");
        // loop and display all local nodes
    }
}

impl AccountRoute {
    #[allow(unused_variables)]
    pub fn create(args: Vec<String>) {
        // generate public, private keys and address

        // display public private keys and address
    }

    pub fn get() {
        // get all accounts from local database

        // display all accounts
    }

    pub fn current() {
        // get current account from local database

        // display current account
    }
}

impl BlockchainRoute {
    pub fn list() {
        // use a loop to list all blocks in the local database
    }
}

impl TransactionRoute {
    pub fn list() {
        // list all transactions from the local database
    }

    #[allow(unused_variables)]
    pub fn transfer(args: Vec<String>) {
        // perform a transfer from Node A to Node B

        // display the unblock spread

        // display the transaction details
    }
}

impl MinerRoute {
    #[allow(unused_variables)]
    pub fn start(args: Vec<String>) {
        // check if there is a current account
        // throw error "to create account" if no account exists

        // start a node with the current account
        // in an infinite loop, mine transactions
    }
}
