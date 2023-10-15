use std::net::{IpAddr, SocketAddr};

// use jsonrpc::simple_tcp::TcpTransport;
use jsonrpc::Client;
use jsonrpc::{Error, Request, Response};
use jsonrpsee_server::ServerHandle;
// use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee_server::{RpcModule, Server};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::{to_raw_value, RawValue};

use crate::database::{BaseDB, BlockchainDB, TransactionDB, UnTransactionDB};
use crate::modules::blockchain::Block;
use crate::modules::node::{add_node, get_nodes};
use crate::modules::transactions::Transaction;

// represent the current node as a RPC Server ready to receive connections
#[derive(Clone, Copy)]
struct RPCServer {
    // server address: `tcp://127.0.0.1:8000
    // server: String,
}

// represent the current node ready to send connections
pub struct RPCClient {
    // server address
    node: String,
    // server client object
    client: Client,
}

struct BroadCast {}

impl BroadCast {
    pub fn ping(args: Vec<String>) {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // interate through all clients
        for client in clients {
            // make RPC call
            client.ping(args.clone());
        }
    }

    fn get_blockchain(args: Vec<String>) -> Result<Vec<Vec<Block>>, Error> {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // storing all blocks from every node
        let mut all_blocks: Vec<Vec<Block>> = vec![];

        // interate through all clients
        for client in clients {
            // make RPC call
            all_blocks.push(client.get_blockchain(args.clone())?);
        }

        Ok(all_blocks)
    }

    fn new_block(block: Block) {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // interate through all clients
        for client in clients {
            // make RPC call
            client.new_block(block.clone());
        }
    }

    fn add_node(address: String) {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // interate through all clients
        for client in clients {
            // make RPC call
            client.add_node(address.clone())?;
        }
    }

    pub fn get_transactions(args: Vec<String>) -> Result<Vec<Vec<Transaction>>, Error> {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();
        let mut all_transactions: Vec<Vec<Transaction>> = vec![];

        // interate through all clients
        for client in clients {
            // make RPC call
            all_transactions.push(client.get_transactions(args.clone())?);
        }

        Ok(all_transactions)
    }

    fn new_untransaction<T: Copy + Serialize + DeserializeOwned>(args: T) {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // interate through all clients
        for client in clients {
            // make RPC call
            client.new_untransaction(args.clone());
        }
    }

    fn block_transaction<T: Copy + Serialize + DeserializeOwned>(txn: T) {
        // collect all nodes from local database
        let clients: Vec<RPCClient> = get_clients();

        // interate through all clients
        for client in clients {
            // make RPC cal
            client.block_transaction(txn.clone());
        }
    }
}

impl RPCServer {
    fn new() -> RPCServer {
        // return an initialized RPC Server
        RPCServer {
            // server
        }
    }

    /// Check for connectivity
    fn ping(&self) -> bool {
        // indicate connectivity
        true
    }

    /// Get blockchain from local database
    fn get_blockchain(&self) -> Vec<Block> {
        // return all block from the local blockchain database
        BlockchainDB::new().find_all()
    }

    /// Add a new block to the local database
    fn new_block<T: Serialize + DeserializeOwned>(&self, block: T) -> () {
        // insert a new block to the blockchain database
        BlockchainDB::new().insert(block).unwrap();

        // clear database that stores transactions to be mined
        UnTransactionDB::new().clear().unwrap();

        println!("Received New Block")
    }

    /// Add a node to the local database
    fn add_node(&self, address: String) {
        // add node to local database cloning `address` as mutable string
        add_node(address)
    }

    /// Get all transactions from the local database
    fn get_transactions(&self) -> Vec<Transaction> {
        // return all transactions from local database
        println!("Transactions ----");
        TransactionDB::new().find_all()
    }

    /// Add an un-mined transaction to the local database
    fn new_untransaction<T: Serialize + DeserializeOwned>(&self, untxns: T) -> () {
        // TODO: What if it fails to insert the transaction?
        UnTransactionDB::new().insert(untxns).unwrap()
    }

    /// Write a new mined transaction to the local database
    fn block_transaction<T: Serialize + DeserializeOwned>(&self, txns: T) -> () {
        println!("Received new block transaction!");

        // TODO: What if it fails to write a transaction?
        TransactionDB::new().write(txns).unwrap()
    }
}

impl RPCClient {
    pub fn new(node: String) -> RPCClient {
        // If any, strip scheme from address
        let stripped_node: String = node.strip_prefix("http://").unwrap_or(&node).to_string();

        // split the node address `127.0.0.1:8000` to `127.0.0.1` and `8080`
        let address_split: Vec<&str> = stripped_node.split(":").collect();

        // parse String to IP address
        let addr: IpAddr = address_split[0].parse().unwrap();

        // parse String to port
        let port: u16 = address_split[1].parse::<u16>().unwrap();

        // construct a RPC client
        RPCClient {
            node,
            // construct client with transport tcp transport protocol
            client: Client::simple_http("http://127.0.0.1:8332", None, None).unwrap(),
        }
    }

    pub fn ping(&self, args: Vec<String>) -> Result<bool, Error> {
        // serialize arguments to raw json
        let params = [to_raw_value(&args)?];

        // build request with parameters
        let request: Request = self.client.build_request("ping", &params);
        println!("About sending request");
        // send request
        let response: Response = self.client.send_request(request)?;

        println!("Request sent");
        // deserialize response or an error
        response.result::<bool>()
    }

    fn get_blockchain(&self, args: Vec<String>) -> Result<Vec<Block>, Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] = [to_raw_value(&args)?];

        // build request with parameters
        let request: Request = self.client.build_request("get_blockchain", &params);

        // send request
        let response: Response = self.client.send_request(request)?;
        // deserialize response or return an error
        response.result::<Vec<Block>>()
    }

    fn new_block(&self, block: Block) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] = [to_raw_value(&block)?];

        // construct request with parameters
        let request: Request = self.client.build_request("new_block", &params);

        // send request || doesn't require a response
        self.client.send_request(request)?;

        // print debug message
        println!("Sent new block");
        Ok(())
    }

    fn add_node(&self, address: String) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] = [to_raw_value(&address)?];

        // construct request with parameters
        let request: Request = self.client.build_request("add_node", &params);

        // send request || doesn't require a respnse
        self.client.send_request(request)?;

        println!("Adding node {address} to network");

        Ok(())
    }

    pub fn get_transactions(&self, args: Vec<String>) -> Result<Vec<Transaction>, Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] = [to_raw_value(&args)?];

        // construct request with parameters
        let request: Request = self.client.build_request("get_transactions", &params);
        println!("Sending Transactions");
        let response: Response = self.client.send_request(request)?;

        // deserialize response or throw error
        response.result::<Vec<Transaction>>()
    }

    fn new_untransaction<T: Serialize + DeserializeOwned>(&self, args: T) -> Result<(), Error> {
        // serialize arguments to raw json
        let params: [Box<RawValue>; 1] = [to_raw_value(&args)?];

        // construct request with parameters
        let request: Request = self.client.build_request("new_untransaction", &params);

        // send request || doesn't require response
        self.client.send_request(request)?;

        Ok(())
    }

    fn block_transaction<T: Serialize + DeserializeOwned>(&self, txn: T) -> Result<(), Error> {
        // serialize arguments to json
        let params: [Box<RawValue>; 1] = [to_raw_value(&txn)?];

        // construct request with parameters
        let request: Request = self.client.build_request("block_transaction", &params);

        self.client.send_request(request)?;

        Ok(())
    }
}

// Returns an iterable RPCClient(s)
fn get_clients() -> Vec<RPCClient> {
    // placeholder to store queried clients
    let mut clients: Vec<RPCClient> = Vec::new();

    // query local database for available nodes
    let nodes: Vec<String> = get_nodes();

    // iterate through all nodes
    for node in nodes {
        // construct RPC client from the node
        clients.push(RPCClient::new(node));
    }

    // return clients
    clients
}

// ip: String, port: u16

pub async fn start_server() -> Result<(), Box<dyn std::error::Error>> {
    // initialize RPC Server
    let rpc_server: RPCServer = RPCServer::new();

    // initialize Input/Output handler
    let mut io: RpcModule<()> = RpcModule::new(());

    // registering RPCServer methods
    io.register_method("ping", move |_, _| rpc_server.ping())?;
    io.register_method("get_blockchain", move |_, _| rpc_server.get_blockchain())?;

    io.register_method("new_block", move |_, params| {
        // check deserialization results
        match serde_json::from_value::<Block>((*params).into()) {
            // if deserialization is successful
            Ok(block) => rpc_server.new_block(block),
            Err(_) => (),
        };
    })?;

    io.register_method("add_node", move |_, params| {
        // check deserialization results
        match serde_json::from_value::<String>((*params).into()) {
            // if deserialization is successful
            Ok(node) => rpc_server.add_node(node),
            Err(_) => (),
        };
    })?;

    io.register_method("get_transactions", move |_, _| {
        rpc_server.get_transactions()
    })?;

    io.register_method("block_transactions", move |_, params| {
        // deserialize parameter to Transaction or Vec<Transactions>
    })?;

    // Create a server instance and bind
    let server: Server = Server::builder()
        .build("0.0.0.0:8332".parse::<SocketAddr>()?)
        .await?;

    // start the server
    println!("{:?}", &server.local_addr());
    let handle: ServerHandle = server.start(io);

    println!("Server running at 0.0.0.0:8332");

    // handle the server shutdown gracefully
    handle.stopped().await;

    Ok(())
}
