use reqwest::Client;
use serde::Deserialize;


const POLYGON_API_URL: &str = "https://api.polygon.io/v2/aggs/ticker";
const API_KEY: &str = "JEXTjx_z3ADb4bnBnoFZoKO50gp2Z5vi";


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

    pub async fn fetch_data(&self, symbol: &str, from: &str, to: &str) -> Result<AggregateResponse, reqwest::Error> {
        let url = format!("{}/{}/range/1/day/{}/{}?adjusted=true&apiKey={}", POLYGON_API_URL, symbol, from, to, API_KEY);
        //println!("Fetching data from {} to {}", from, to);
        let response = self.client.get(&url).send().await?;
        //println!("Response: {:?}", response);
        let data: AggregateResponse = response.json().await?;
        //println!("Data: {:?}", data);
        Ok(data)
    }
}