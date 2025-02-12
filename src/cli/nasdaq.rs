use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MarketData {
    symbol: String,
    name: String,
    lastsale: String,
    netchange: String,
    pctchange: String,
    #[serde(rename = "marketCap")] // name from the nasdaq api
    market_cap: Option<String>,
    country: String,
    ipoyear: Option<String>,
    volume: String,
    sector: String,
    industry: String,
    url: String,
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
