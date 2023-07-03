pub struct Account {}

pub struct Blockchain {}

pub struct Miner {}

pub struct Node {}

pub struct Transaction {}

impl Node {
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

impl Account {
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

impl Blockchain {
    pub fn list() {
        // use a loop to list all blocks in the local database
    }
}

impl Transaction {
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

impl Miner {
    #[allow(unused_variables)]
    pub fn start(args: Vec<String>) {
        // check if there is a current account
        // throw error "to create account" if no account exists

        // start a node with the current account
        // in an infinte loop, mine transactions
    }
}
