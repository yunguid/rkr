use reqwest::Client;
use serde::Deserialize;

const ALPHA_VANTAGE_API_KEY: &str = "https://www.alphavantage.co/query";
CONST API_KEY: &STR = "JXRNYVYBVD826WVE"

#[derive(Deserialize, Debug)]
struct StockData {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

#[derive(Deserialize, Debug)]
struct AlphaVantageResponse {
    #[serde(rename = "Time Series (Daily)")]
    time_series_daily: std::collections::HashMap<String, StockData>,
}

pub struct DataProvider {
    client: Client,
}
/*
    This module is responsible for fetching market data from an external API
    and providing it to the rest of the system.
    DataProvider struct has two fields:
    - client: A reqwest::Client instance used to make HTTP requests
    - base_url: The base URL of the API to fetch market data from
*/
impl DataProvider {
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn fetch_data(&self) -> Result<String, reqwest::Error> {
        let response = self.client.get(&self.base_url).send().await?;
        let body = response.text().await?;
        Ok(body)
    }
}