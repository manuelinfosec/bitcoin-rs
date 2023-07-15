use serde::{Deserialize, Serialize};

use database::{BaseDB, NodeDB};

use crate::database;

#[derive(Debug, Serialize, Deserialize)]
pub struct Nodes(pub Vec<String>);

pub fn get_nodes() -> Vec<String> {
    NodeDB::new().find_all()
}

pub fn add_node(address: &mut String) {
    // define node schema
    let schema: &str = "tcp://";
    let node_db: NodeDB = NodeDB::new();

    // collect all nodes from the local database
    let mut all_nodes: Vec<String> = node_db.find_all();

    // check if address contains a schema
    if !address.contains(schema) {
        address.push_str(schema)
    }

    // append address to list of nodes
    all_nodes.push(address.clone());

    // clear local node database
    node_db.clear();

    // write all nodes back to local database
    // TODO: Sort the nodes before writing to database
    node_db.write(all_nodes);
}