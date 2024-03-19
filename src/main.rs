use std::sync::Arc;
use tokio::sync::Mutex;
use log::info;
use env_logger;

mod data_ingestion;
/* 
This is the main entry point for the RKR trading system. It is responsible for
spawning tasks for different modules and keeping the main task running.
Create a shared state for market dta and spawns a task for data ingestion.
*/
#[tokio::main]
async fn main() {
    // Create a shared state for storing market data
    // Uses Arc(Atomic Reference Counting) to allow multiple ownership and Mutex for thread-safe access to data
    env_logger::init();
    info!("Staring RKR trading system...");
    let market_data = Arc::new(Mutex::new(Vec::new()));
    let market_data_clone = Arc::clone(&market_data);


    // Spawn a task for data ingestion
    
    tokio::spawn(async move {
        data_ingestion::fetch_market_data(market_data_clone).await;
    });

    // Keep the main task running
    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl-c");
    println!("Shutting down the RKR trading system...");
}