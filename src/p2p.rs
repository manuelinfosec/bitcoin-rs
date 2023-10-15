use std::net::{IpAddr, SocketAddr};

// use jsonrpc::simple_tcp::TcpTransport;
use jsonrpc::Client;
use jsonrpc::{Error, Request, Response};
use jsonrpsee_server::ServerHandle;
// use jsonrpsee::server::{RpcModule, Server};
use jsonrpsee_server::{RpcModule, Server};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::value::{to_raw_value, RawValue, Value};

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

impl BroadCast {}

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
        println!("test ping");
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
        add_node(&mut address.clone())
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

        println!("{} {}", addr, port);

        // Defining the transport protocol to use tcp 1.0 is used because minimal dependencies
        // ...is the goal and it's ok to use synchronous communication

        // let transport: TcpTransport = TcpTransport::new(SocketAddr::new(addr, port));

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
        self.client.send_request(request);

        Ok(())
    }

    fn block_transaction<T: Serialize + DeserializeOwned>(&self, txn: T) -> Result<(), Error> {
        // serialize arguments to json
        let params: [Box<RawValue>; 1] = [to_raw_value(&txn)?];

        // construct request with parameters
        let request: Request = self.client.build_request("block_transaction", &params);

        self.client.send_request(request);

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
    println!("Starting server?");
    // initialize RPC Server
    let rpc_server: RPCServer = RPCServer::new();

    // initialize Input/Output handler
    let mut io: RpcModule<()> = RpcModule::new(());

    // add the ping method to handler
    // io.add_sync_method("ping", move |_| async {
    //     Ok(Value::Bool(rpc_server.ping().await))
    // });

    // registering RPCServer methods
    io.register_method("ping", move |_, _| rpc_server.ping())?;
    io.register_method("get_blockchain", move |_, _| rpc_server.get_blockchain())?;

    io.register_method("new_block", move |_, params| {
        // deserialize `params` to Block
    })?;

    io.register_method("add_node", move |_, params| {
        // check deserialization results
        match serde_json::from_value::<String>((*params).into()) {
            // if deserializ
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

    println!("Server running at 0.0.0.0:8332");

    // Create a server instance and bind
    let server: Server = Server::builder()
        .build("0.0.0.0:8332".parse::<SocketAddr>()?)
        .await?;

    // start the server
    println!("{:?}", &server.local_addr());
    let handle: ServerHandle = server.start(io);

    println!("Server Started");

    // handle the server shutdown gracefully
    handle.stopped().await;

    Ok(())
}
