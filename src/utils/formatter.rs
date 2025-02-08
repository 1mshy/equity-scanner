use crate::utils;

pub const YAHOO_BASE_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart/";

pub fn historical_url(ticker: &str) -> String {
    format!(
        "{}{}?period1=0&period2={}&interval=1d",
        YAHOO_BASE_URL,
        ticker,
        utils::time::current_unix_time()
    )
}
