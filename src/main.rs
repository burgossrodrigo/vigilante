/**
 * @dev Imports info
 * ethers - the crate
 * providers - a module within the crate ethers
 * Provider - item within the providers module
 * Http - same as above
 */
use ethers::{
    providers::{Http, Middleware, Provider, StreamExt},
    types::Transaction,
};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

/**
 * @dev Imports info
 * tokyo - ideal to work with assyncronous code
 */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Provider::<Http>::try_from("http://34.29.78.250:8545")
        .expect("Provider initialization failed");
    let provider = Arc::new(Mutex::new(provider));
    let target_address = "0xC36442b4a4522E871399CD717aBDD847Ab11FE88".to_lowercase();

    // /**
    //  * @dev explanation over mpsc
    //  * Multiple producers, single consumers
    //  * Used to send message through multiple threads
    //  * @param 32 - number of messages the channel can handle
    //  */
    let (tx, mut rx) = mpsc::channel(32);

    let provider_clone = Arc::clone(&provider);

    tokio::spawn(async move {
        let mut provider = provider_clone.lock().await;
        let mut stream = provider.watch_blocks().await.unwrap();
        while let Some(block_hash) = stream.next().await {
            match provider.get_block_with_txs(block_hash).await {
                Ok(Some(block)) => {
                    for tx_in_block in block.transactions {
                        if tx_in_block.to.expect("REASON").to_string().to_lowercase() == target_address || 
                            tx_in_block.from.to_string().to_lowercase() == target_address
                        {
                            if tx.send(tx_in_block).await.is_err() {
                                eprintln!("Failed to send matching block; receiver might have been dropped.");
                                break;
                            }
                        }
                    }
                }
                Ok(none) => eprint!("Block not found for hash: {:?}", block_hash),
                Err(e) => eprintln!("Error fetching block: {:?}", e),
            }
        }
    });

    while let Some(tx_in_block) = rx.recv().await {
        println!("New block: {:?}", tx_in_block.hash.to_string().to_lowercase())
    }

    Ok(())
}
