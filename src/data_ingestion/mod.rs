use crate::data_ingestion::data_provider::AggregateResponse;

pub mod data_provider;


use data_provider::DataProvider;
use log::{info, error};

pub async fn fetch_market_data(symbols: &[String], start_date: &str, end_date: &str) -> Result<Vec<AggregateResponse>, reqwest::Error> {
    let provider = DataProvider::new();
    let mut market_data = Vec::new();

    for symbol in symbols {
        match provider.fetch_data(symbol, start_date, end_date).await {
            Ok(data) => {
                info!("Fetched market data for {}", symbol);
                market_data.push(data);
            }
            Err(e) => {
                error!("Failed to fetch market data: {}", e);
            }
        }
    }
    Ok::<Vec<AggregateResponse>, reqwest::Error>(market_data)
}

