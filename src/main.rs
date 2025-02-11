#![allow(unused)]

mod client;
mod utils;

use client::YahooFinanceClient;
use std::time::{SystemTime, UNIX_EPOCH};
#[tokio::main]

async fn main() {
    let mut client = YahooFinanceClient::new().await.unwrap();
    let output = client.fetch_quote_summary("AAPL").await.unwrap();

    let mut history = client.fetch_historical("aapl").await.unwrap();
    println!("Today's price: {:?}", history.close[history.close.len()-1]);
    // println!("{:?}", history.volume);
    client.analyse(&history).await;
    let current_rsi = client.current_rsi(&history, 14).expect("Rsi cannot be calculated");
    println!("Current rsi: {:?}", current_rsi);
    // println!("{:?}", client);
    // println!("{:?}", output);
}
