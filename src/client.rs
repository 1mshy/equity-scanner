use reqwest::Client;
use serde_json::Value;
use std::error::Error;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct YahooFinanceClient {
    client: Client,
    crumb: Option<String>,
    last_refresh: Option<Instant>,
}

impl YahooFinanceClient {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let client = Client::builder()
            .cookie_store(true)
            .https_only(true)
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36")
            .build()?;

        Ok(YahooFinanceClient {
            client,
            crumb: None,
            last_refresh: None,
        })
    }

    pub async fn refresh_crumb(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.get("https://fc.yahoo.com").send().await?;

        let crumb = self
            .client
            .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
            .send()
            .await?
            .text()
            .await?;

        self.crumb = Some(crumb);
        self.last_refresh = Some(Instant::now());
        Ok(())
    }

    pub async fn ensure_crumb_valid(&mut self) -> Result<(), Box<dyn Error>> {
        let crumb_ttl = Duration::from_secs(15 * 60); // 15 minutes
        match (self.crumb.as_ref(), self.last_refresh) {
            (Some(_), Some(t)) if t.elapsed() < crumb_ttl => Ok(()),
            _ => self.refresh_crumb().await,
        }
    }

    pub async fn fetch_url_summary(&mut self, url: &str) -> Result<Value, Box<dyn Error>> {
        self.ensure_crumb_valid().await?;
        let crumb = self.crumb.as_ref().ok_or("Crumb not found")?;
        let full_url = format!("{}&crumb={}", url, crumb);

        let response = self
            .client
            .get(&full_url)
            .header("Accept", "application/json")
            .send()
            .await?;

        // if response.status().as_u16() == 403 {
        //     // Crumb might be invalid, refresh and retry once
        //     self.refresh_crumb().await?;
        //     let crumb = self.crumb.as_ref().ok_or("Crumb not found after refresh")?;
        //     let full_url = format!("{}&crumb={}", url, crumb);
        //     let retry_response = self
        //         .client
        //         .get(&full_url)
        //         .header("Accept", "application/json")
        //         .send()
        //         .await?;
        //     let text = retry_response.text().await?;
        //     return Ok(serde_json::from_str(&text)?);
        // }

        let text = response.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    pub async fn fetch_quote_summary(&mut self, symbol: &str) -> Result<Value, Box<dyn Error>> {
        self.ensure_crumb_valid().await?;

        let crumb = self.crumb.as_ref().ok_or("Crumb not found")?;
        let url = format!(
            "https://query1.finance.yahoo.com/v10/finance/quoteSummary/{}?modules=assetProfile%2CfinancialData&crumb={}",
            symbol, crumb
        );

        self.fetch_url_summary(&url).await
    }
}
