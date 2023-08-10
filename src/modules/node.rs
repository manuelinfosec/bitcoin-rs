use serde::{Deserialize, Serialize};

use database::{BaseDB, NodeDB};

use crate::database;
use crate::p2p::{RPCClient, start_server};

/// Get all nodes from local database
// pub fn get_nodes() -> Vec<String> {
//     NodeDB::new().find_all()
// }

pub fn get_nodes() {
    let client: RPCClient = RPCClient::new("127.0.0.1:8332".to_string());

    println!("About to be pinging...");
    let result: bool = client.ping(vec![]).expect("Could not ping user");
    println!("Result: {result}");
    println!("Pinging too..");
}

/// Add a node to the local database
pub fn add_node(address: &mut String) {
    // Initialize local database API
    let node_db: NodeDB = NodeDB::new();

    // define address schema
    let schema: &str = "tcp://";

    // check if address contains a schema
    if !address.contains(schema) {
        // append to the beginning of the string
        address.insert_str(0, schema);
    }

    // TODO: Sort nodes before writing to local database

    // write all nodes to local database
    node_db.write(address.clone())
        .expect("Couldn't write to Node database");
}

/// Perform all due diligence to make the current node blockchain-ready
fn init_node() {
    // collect blockchain from all nodes on the network

    // collect all the transactions from all nodes on the network

    // initialize local Transaction and Blockchain databases

    // get blockchain and transactions from local database

    // iterate through the node_blockchains
    // if there a blockchain downloaded that is longer than what we have locally, replace local.


    // iterate through the node_transactions
    // if there are transactions downloaded that is longer than what we have locally, replace local.
}

pub fn start_node(address: String) {
    // make the current node blockchain-ready
    init_node();

    println!("Node initialization success");

    // check if schema exists or add schema to address

    // thread the local RPC server
    start_server();
}