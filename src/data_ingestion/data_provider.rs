use reqwest::Client;
use serde::Deserialize;
use anyhow::anyhow;
use anyhow::{Result, Error};
use reqwest::Error as ReqwestError;
use std::env;

const POLYGON_API_URL: &str = "https://api.polygon.io/v2/aggs/ticker";

#[derive(Deserialize, Debug)]
pub struct AggregateResponse {
    pub ticker: String,
    pub adjusted: bool,
    pub results: Vec<Aggregate>,
}

#[derive(Deserialize, Debug)]
pub struct Aggregate {
    pub c: f64,
    pub h: f64,
    pub l: f64,
    pub o: f64,
    pub t: i64,
    pub v: f64,
}

pub struct DataProvider {
    client: Client,
}

impl DataProvider {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_data(&self, symbol: &str, from: &str, to: &str) -> Result<AggregateResponse, Error> {
        let api_key = env::var("POLYGON_API_KEY").expect("POLYGON_API_KEY must be set");
        let url = format!("{}/{}/range/1/day/{}/{}?adjusted=true&apiKey={}", POLYGON_API_URL, symbol, from, to, api_key);
        println!("Fetching data for {} from {} to {}", symbol, from, to);
        let response = self.client.get(&url).send().await?;
    
        println!("Response: {:?}", response);
        if response.status().is_success() {
            let data: AggregateResponse = response.json().await?;
            println!("Data: {:?}", data);
            Ok(data)
        } else {
            let status = response.status();
            let error_message = response.text().await?;
            println!("Error fetching data for {}: {}", symbol, error_message);
            Err(anyhow!("Error fetching data for {}: {}", symbol, error_message))
        }
    }
}