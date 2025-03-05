use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    pub symbol: String,
    pub name: String,
    pub lastsale: String,
    pub netchange: String,
    pub pctchange: String,
    #[serde(rename = "marketCap")] // name from the nasdaq api
    pub market_cap: Option<String>,
    pub country: String,
    pub ipoyear: Option<String>,
    pub volume: String,
    pub sector: String,
    pub industry: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    data: Option<ApiData>,
}

#[derive(Debug, Deserialize)]
struct ApiData {
    rows: Vec<MarketData>,
}

pub async fn market_overview() -> Result<Vec<MarketData>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("PostmanRuntime/7.43.0"),
    );

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.nasdaq.com/api/screener/stocks?tableonly=true&offset=0&download=true")
        .headers(headers)
        .send()
        .await?;

    let api_response: ApiResponse = response.json().await?;

    Ok(api_response.data.map_or_else(Vec::new, |d| d.rows))
}

pub fn filter(
    data: Vec<MarketData>,
    min_market_cap: Option<u64>,
    first: Option<u64>,
) -> Vec<MarketData> {
    let min_market_cap = min_market_cap.unwrap_or(0) as f64;
    let new_data: Vec<MarketData> = data
        .into_iter()
        .filter(|ticker_data| {
            ticker_data
                .market_cap
                .as_ref()
                .and_then(|market_cap| market_cap.parse::<f64>().ok())
                .map_or(false, |market_cap| market_cap > min_market_cap)
        })
        .collect();
    new_data
}

pub fn market_cap_filter(data: Vec<MarketData>, min_market_cap: f64) -> Vec<MarketData> {
    let new_data: Vec<MarketData> = data
        .into_iter()
        .filter(|ticker_data| {
            ticker_data
                .market_cap
                .as_ref()
                .and_then(|market_cap| market_cap.parse::<f64>().ok())
                .map_or(false, |market_cap| market_cap > min_market_cap)
        })
        .collect();

    new_data
}
