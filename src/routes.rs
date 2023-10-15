use crate::modules::node;
use crate::modules::node::{get_nodes, start_node};

pub struct AccountRoute {}

pub struct BlockchainRoute {}

pub struct MinerRoute {}

pub struct NodeRoute {}

pub struct TransactionRoute {}

impl NodeRoute {
    #[allow(unused_variables)]
    pub fn add(args: Vec<String>) {
        let mut node: String = args[3].clone();

        // add the node locally
        node::add_node(node);

        // broadcast to all local nodes
    }

    pub fn list() {
        println!("Querying for local nodes...");
        // // query all local nodes to vector
        // let nodes: Vec<String> = get_nodes();
        //
        // // iterate and write all local nodes to STDOUT
        // for node in nodes {
        //     println!("{node}");
        // }
        get_nodes();
        ()
    }

    #[allow(unused_variables)]
    pub async fn start(args: Vec<String>) {
        // collect address from arguments
        // let addr: String = args[3].to_string();
        let addr: String = String::new();

        println!("Starting node at {addr}");

        // start a node
        start_node(addr).await
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
