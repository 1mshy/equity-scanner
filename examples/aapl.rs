#![allow(unused)]

use equity_scanner::{
    client::{self, YahooFinanceClient},
    equity::Equity,
    utils::structs::HistoricalData,
};
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]

async fn main() {
    let mut client = YahooFinanceClient::new().await.unwrap();
    let output = client.fetch_quote_summary("AAPL").await.unwrap();

    let mut equity = client.fetch_historical("AAPL").await.unwrap();
    println!(
        "Today's price: {:?}",
        equity.historical_data.close[equity.historical_data.close.len() - 1]
    );
    let current_rsi = equity.current_rsi(14);
    println!("Current rsi: {:?}", current_rsi);

    println!("Is overbought:{}", equity.is_overvalued())

    // println!("{:?}", client);
    // println!("{:?}", output);
}
