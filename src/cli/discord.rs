use std::{thread::sleep, time::Duration};

use serde::Deserialize;
use webhook::client::WebhookClient;

const IMAGE_URL: &str = "https://imgs.search.brave.com/l9f00WuduVvIDIWPmchBqoorwMrPAipevjqxy16CH3Q/rs:fit:860:0:0:0/g:ce/aHR0cHM6Ly9tZWRp/YS5nZXR0eWltYWdl/cy5jb20vaWQvMTQ3/Mzc3MTY0Ni9waG90/by9hLXlvdW5nLW1h/bi1idXlzLWEtbmV3/LWNhci5qcGc_cz02/MTJ4NjEyJnc9MCZr/PTIwJmM9b0xiaTJr/ZVNnOGc4LUdTbEpR/MHdxSHotbEFodDhy/cXhhRVF5dml1eURQ/az0"; // Use a relevant image

pub async fn send_stock_message(url: &str, ticker: &str, price: f64, price_change: f64, rsi: f64) {
    let price_emoji = if price_change > 0.0 {
        "📈"
    } else if price_change < 0.0 {
        "📉"
    } else {
        "⚖️"
    };

    let rsi_trend = if rsi > 70.0 {
        "Overbought 🚨"
    } else if rsi < 30.0 {
        "Oversold 📉"
    } else {
        "Neutral 🎯"
    };

    let client: WebhookClient = WebhookClient::new(url);
    let discord_request = client
        .send(|message| {
            message
                .username("Stock Bot 🚀")
                .avatar_url(IMAGE_URL)
                .embed(|embed| {
                    embed
                        .title(&format!("📊 Stock Update: {} {}", ticker, price_emoji))
                        .description(&format!(
                            "**Latest Price:** ${:.2} {}\n**RSI:** {:.2} ({})",
                            price, price_emoji, rsi, rsi_trend
                        ))
                        // .color(if price_change >= 0.0 { "green" } else { "red" }) // Green for up, red for down
                        .footer(
                            "💡 Market insights powered by Yahoo Finance",
                            Some(String::from(IMAGE_URL)),
                        )
                        // .image(IMAGE_URL)
                        // .thumbnail(IMAGE_URL)
                        .field(
                            "💰 Price",
                            &format!(
                                "${:.2} ({:.2}%)",
                                price,
                                (price_change / price) * 100.0
                            ),
                            true,
                        )
                        .field("📊 RSI", &format!("{:.2} ({})", rsi, rsi_trend), true)
                        .field(
                            "📅 Timestamp",
                            &format!("{:?}", std::time::Instant::now()),
                            false,
                        )
                })
        })
        .await;

    match discord_request {
        Ok(_) => println!("Discord request successfull for {}", ticker),
        Err(e) => {
            println!("Error with discord request:\n{}", e);
            let message = e.to_string();
            println!("message");
            println!("{}", message);
            let json_message = 
            match serde_json::from_str::<RateLimited>(&message) {
                Ok(json_message) => json_message,
                Err(_) => {
                    return;
                }   
            };
            let wait_time = json_message.retry_after;
            println!("Going to wait {} due to discord rate limit", wait_time);
            sleep(Duration::from_secs_f64(wait_time));
            let _ = send_stock_message(url, ticker, price, price_change, rsi);
        }
    };
}
#[allow(dead_code)]
#[derive(Deserialize)]
struct RateLimited {
    message: String,
    retry_after: f64,
    global: bool,
}