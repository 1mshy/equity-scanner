#![allow(unused)]

mod client;
mod utils;

use client::YahooFinanceClient;
use std::time::{SystemTime, UNIX_EPOCH};
#[tokio::main]

async fn main() {
    let mut client = YahooFinanceClient::new().await.unwrap();
    let output = client.fetch_quote_summary("AAPL").await.unwrap();

    let history = client.fetch_historical("AAPL").await.unwrap();
    println!("{:?}", history.metadata);
    // println!("{:?}", client);
    // println!("{:?}", output);
}
