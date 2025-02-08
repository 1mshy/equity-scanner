#![allow(unused)]

use client::YahooFinanceClient;

mod client;
#[tokio::main]

async fn main() {
    let mut client = YahooFinanceClient::new().await.unwrap();
    let output = client.fetch_quote_summary("AAPL").await.unwrap();
    println!("{:?}", client);
    println!("{:?}", output);
}
