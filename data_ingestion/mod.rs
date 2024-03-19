pub mod data_provider;

use std::sync::Arc;
use tokio::sync::Mutex;
use data_provider::DataProvider;
use log::{info, error};

pub async fn fetch_market_data(market_data: Arc<Mutex<Vec<String>>>) {
    let provider = DataProvider::new();
    let symbol = "AAPL"; // Replace with the desired stock symbol

    loop {
        match provider.fetch_data(symbol).await {
            Ok(data) => {
                let mut locked_data = market_data.lock().await;
                locked_data.push(format!("{:?}", data));
                info!("Fetched market data for {}", symbol);
            }
            Err(e) => {
                error!("Failed to fetch market data: {}", e);
            }
        }

        // Sleep for a certain interval before fetching again
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;
    }
}