pub mod config;
pub mod discord;
pub mod nasdaq;

use std::time::Duration;

use clap::{Parser, Subcommand};
use config::Config;
use discord::send_stock_message;
use equity_scanner::client::YahooFinanceClient;
use nasdaq::market_overview;
use tokio::time::sleep;

#[derive(Parser)]
#[command(name = "equity-scanner")]
#[command(about = "A Rust-based equity scanning CLI tool", long_about = None)]
#[command(author, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(short, long)]
    webhook: Option<String>,
}
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        symbol: String,
    },
    Set {
        symbol: String,
        rsi_upper_limit: f32,
        rsi_lower_limit: f32,
    },
    All,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    let market_data = match market_overview().await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error fetching market data: {}", err);
            Vec::new()
        }
    };
    let mut config = Config::new();
    let mut client = YahooFinanceClient::new().await.unwrap();

    if let Some(webhook_url) = &args.webhook {
        println!("Using webhook: {}", webhook_url);
        config.set_webhook(webhook_url);
        println!("Set webhook as default in config");
    }

    match args.cmd {
        Commands::All => {
            for row in market_data {
                let symbol = row.symbol;
                println!("{}", symbol);
                let mut equity_data = match client.fetch_historical(&symbol).await {
                    Ok(equity_data) => equity_data,
                    Err(_) => {
                        continue;
                    }
                };
                let equity_config = config.get_equity(&symbol);

                let current_rsi = match equity_data.current_default_rsi() {
                    Ok(current_rsi) => current_rsi as f32,
                    Err(e) => {
                        println!("Error fetching data for {}, skipping", symbol);
                        println!("{}", e);
                        continue;
                    }
                };

                if current_rsi < equity_config.rsi_lower_limit
                    || current_rsi > equity_config.rsi_upper_limit
                {
                    let summary = client.fetch_quote_summary(&symbol).await.unwrap();
                    println!("{:#?}", summary);
                    let webhook_url = match &config.get_webhook() {
                        Some(webhook) => webhook.clone(),
                        None => {
                            println!("No webhook found, thus cannot send webhook message.");
                            println!("Do some digging on {}", &symbol);
                            continue;
                        }
                    };
                    send_stock_message(&webhook_url, &symbol, 100.0, 50.0, current_rsi as f64).await;
                }
                sleep(Duration::from_secs(1)).await;
            }
        }
        Commands::Get { symbol } => {
            let equity = config.get_equity(&symbol);
            match client.fetch_historical(&symbol).await {
                Ok(data) => {
                    println!("{}", data)
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
            println!("Config:\n{}", equity);
        }
        Commands::Set {
            symbol,
            rsi_upper_limit,
            rsi_lower_limit,
        } => {
            config.set_equity(&symbol, rsi_upper_limit, rsi_lower_limit);
            let aapl_config = config.get_equity(&symbol);
            println!("{}", aapl_config);
        }
    }
}
