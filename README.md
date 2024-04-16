# Vigilante - Ethereum Block Watcher

## Overview
This project provides a Rust-based tool for monitoring Ethereum blockchain transactions. It connects to an Ethereum node via HTTP and listens for new blocks. When a new block is found, the application checks each transaction within the block to see if it involves a specific Ethereum address as either the sender (from) or the receiver (to). If a transaction matches the criteria, its hash is logged.

## Requirements
Rust Programming Language: Ensure Rust is installed on your system. You can download and install Rust via rustup.
Ethereum Node: An accessible Ethereum HTTP RPC endpoint is required. This can be a local node (e.g., running via Geth or OpenEthereum) or a remote service (e.g., Infura, Alchemy).

## Setup and Installation

### Clone the Repository:

```
git clone https://github.com/dfb-chain/vigilante.git
cd vigilante
```
### Build the Project:

```
cargo build --release
```

### Run the Application:

```
cargo run --release
```

## Configuration

To configure the Ethereum node endpoint and the target address, modify the following lines in the main.rs file:

```
let provider = Provider::<Http>::try_from("http://0.0.0.0:8545")
    .expect("Failed to initialize provider");
let target_address = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88".to_lowercase();
```

Replace `"http://0.0.0.0:8545"` with your Ethereum node's HTTP endpoint and "0xC36442b4a4522E871399CD717aBDD847Ab11FE88" with the Ethereum address you wish to monitor.

# How It Works

## Main Components

Provider Setup: Establishes a connection to the Ethereum network using an HTTP provider.
Block Watching: Listens for new blocks being added to the blockchain.
Transaction Filtering: Checks each transaction within a block to determine if the to or from address matches the specified target address.
Code Structure
The main function sets up the Ethereum provider and starts an asynchronous task that listens for new blocks. Each block's transactions are examined, and matching transactions are sent to the main thread for logging:

```
tokio::spawn(async move {
    // Locking the provider and starting to watch blocks
    let mut provider = provider_clone.lock().await;
    let mut stream = provider.watch_blocks().await.unwrap();

    // Iterating over each block
    while let Some(block_hash) = stream.next().await {
        // Fetching full block with transactions
        match provider.get_block_with_txs(block_hash).await {
            Ok(Some(block)) => {
                // Checking each transaction
                for tx_in_block in block.transactions {
                    // Comparing addresses
                    if matches_target_address(&tx_in_block, &target_address) {
                        // Send to main thread if matched
                        if tx.send(tx_in_block).await.is_err() {
                            eprintln!("Failed to send transaction; receiver might have been dropped.");
                            break;
                        }
                    }
                }
            },
            // Error handling
            Ok(None) => eprintln!("Block not found for hash: {:?}", block_hash),
            Err(e) => eprintln!("Error fetching block: {:?}", e),
        }
    }
});
```

## Testing

Test the application by running it and observing the console output. Ensure your Ethereum node is fully synchronized to receive real-time block updates.






