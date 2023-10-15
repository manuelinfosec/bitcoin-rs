## Bitcoin-Rust 

Bitcoin-Rust is an educational implementation of the Bitcoin blockchain in Rust, created for the sole purpose of learning and exploration.

This project implements simple blockchain and transactions. Currently, the implementation already has mining, transaction, communication between nodes, and file persistence of blocks and transactions.

The communication between nodes is via RPC based on HTTP, with expansion more complicated implementation of Peer-to-Peer (P2P) networking. Verification of transactions between nodes is based on cryptography

## Usage
(Coming Soon)

## Creating (or joining) Node Network
(Coming Soon)

## Implementation Details
### About Bitcoin Blocks
In Bitcoin, a blockchain is a sequential structure composed of blocks containing transaction data. Each block header is hashed using the SHA-256 cryptographic algorithm to produce a unique hash value. A typical Bitcoin block looks like this:

```json
{
    "size":43560,
    "version":2,
    "previousblockhash":"00000000000000027e7ba6fe7bad39faf3b5a83daed765f05f7d1b71a1632249",
    "merkleroot":"5e049f4030e0ab2debb92378f53c0a6e09548aea083f3ab25e1d94ea1155e29d",
    "time":1388185038,
    "difficulty":1180923195.25802612,
    "nonce":4215469401,
    "tx": [
        "257e7497fb8bc68421eb2c7b699dbab234831600e7352f0d9e6522c7cf3f6c77",
        #...many more transactions omitted...
        "05cfd38f6ae6aa83674cc99e4d75a1458c165b7ab84725eda41d018a09176634"
    ]
}
```

Mining in Bitcoin involves creating a new block by using information from the previous block, such as the parent block hash, timestamp, transaction nonce hash, and a nonce (an incrementing number). After combining this data, a SHA-256 hash is calculated. If the leading digits of the resulting hash consist of several zeroes, the mining difficulty is met. The difficulty adjusts dynamically based on the network's overall mining power, such as:

```
00000000000000027e7ba6fe7bad39faf3b5a83daed765f05f7d1b71a1632249
```

Successful mining. New block found.

### About Bitcoin-rs Blocks
This project has a simplified block data as follows:

```json
{
	"index": 7,
	"timestamp": 1528972070,
	"tx": [
        "b959b3d2099ca304c67087edbf05b79d1f2501b1f407df5e51a1a8c22bb3334d",
        "613e4af7266e01ea338d30681ef606bad26e4cdfa4ec7a6f431e22420c8291fd",
        "be7095a764cb241606a67c9064bc8dbc2da2370d49459bd492473ea5ce304cb3"
    ],
	"previous_block": "00003e17e04d9c9d2c2f5629de20bda58f59af36417a7e50eb77a74a028b026a",
	"nouce": 11063,
	"hash": "00006805c75d0db1685616d9ea5730f6203eda744a16fcc78ef1f3c244083ea4"
}
```

The process of calculating the block hash in Bitcoin-Rust is quite similar to Bitcoin. The project has a relatively low mining difficulty, requiring a hash with just four leading zeros. This design allows for quick and easy mining on standard computers. Unlike Bitcoin, Bitcoin-Rust directly uses an array of transaction hashes instead of a merkle tree.

### About Network
Bitcoin-Rust implements a peer-to-peer (P2P) blockchain network. It uses the RPC (Remote Procedure Call) mechanism provided by (jsonrpsee)[https://github.com/paritytech/jsonrpsee]'s own RPC implementation for simplification.

Nodes in the network can be connected, and they automatically share new transaction information with each other. New nodes will synchronize their blockchain data with existing nodes, ensuring that they have the most up-to-date chain. When a new block is mined, other nodes are notified to update their local copies.

### About Transactions
Bitcoin-Rust follows the Unspent Transaction Output (UTXO) model, which doesn't have a direct concept of "balance." Instead, balances are determined by examining the entire transaction history. Each transaction is composed of one or more inputs and one or more outputs. Bitcoin-Rust supports multiple inputs and outputs in a transaction.

Balances are calculated by summing the unspent transaction outputs (UTXOs) - the outputs of transactions that have not been spent. This is similar to the concept of the UTXO set in Bitcoin.

Transactions that haven't been included in a new block are broadcast to all nodes for verification. Once a miner includes a transaction in a block, it becomes part of the transaction history and is saved in the transaction database.

Please note that transaction correctness checks are currently under development.

## Installation
1. Make sure Rust is installed.
2. Clone the repository:
```bash
git clone https://github.com/manuelinfosec/bitcoin-rs.git
cd bitcoin-rs
```

## Contributing
Contributions are welcome! Please feel free to submit a pull request.