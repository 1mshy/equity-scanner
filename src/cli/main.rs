pub mod nasdaq;

use clap::{Parser, Subcommand};
use nasdaq::{market_overview, MarketData};

#[derive(Parser)]
#[command(name = "equity-scanner")]
#[command(about = "A Rust-based equity scanning CLI tool", long_about = None)]
#[command(author, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
    #[arg(long)]
    webhook: Option<String>,
}
#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
        is_true: bool,
    },
    All,
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    if let Some(webhook_url) = &args.webhook {
        println!("Using webhook: {}", webhook_url)
    }

    let market_data = match market_overview().await {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Error fetching market data: {}", err);
            Vec::new()
        }
    };

    match args.cmd {
        Commands::All => {
            for row in market_data {
                println!("{:#?}", row)
            }
        }
        Commands::Get { key } => {}
        Commands::Set {
            key,
            value,
            is_true,
        } => {}
    }
}
