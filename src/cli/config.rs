use std::{
    collections::HashMap,
    fmt::{self, Display},
    fs::File,
    io::BufReader,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EquityConfig {
    pub symbol: String,
    pub rsi_upper_limit: f32,
    pub rsi_lower_limit: f32,
}
impl Display for EquityConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | RSI {{upper: {}, lower: {}}}",
            self.symbol, self.rsi_upper_limit, self.rsi_lower_limit
        )
    }
}
impl Default for EquityConfig {
    fn default() -> Self {
        Self {
            symbol: String::new(),
            rsi_upper_limit: 70.0,
            rsi_lower_limit: 30.0,
        }
    }
}
impl EquityConfig {
    fn default_symbol(symbol: &str) -> Self {
        EquityConfig {
            symbol: symbol.to_uppercase(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    equities: HashMap<String, EquityConfig>,
    repeat: bool, // Should the problem run in a loop
    webhook_url: Option<String>,
}
// public commands
impl Config {
    pub fn set_equity(&mut self, symbol: &str, rsi_upper_limit: f32, rsi_lower_limit: f32) {
        let equity_config = self
            .equities
            .entry(symbol.to_uppercase())
            .or_default();
        equity_config.symbol = symbol.to_uppercase();
        equity_config.rsi_upper_limit = rsi_upper_limit;
        equity_config.rsi_lower_limit = rsi_lower_limit;
        self.save_config();
    }
    pub fn add_equity(&mut self, equity_config: EquityConfig) {
        self.equities
            .insert(equity_config.symbol.clone(), equity_config);
    }
    /// Get the saved equity config.
    /// If there is none, one will be created and saved.
    pub fn get_equity(&mut self, symbol: &str) -> EquityConfig {
        let equity = match self.equities.get(&symbol.to_uppercase()) {
            Some(equity_config) => equity_config.clone(),
            None => {
                let new_equity = EquityConfig::default_symbol(symbol);
                self.add_equity(new_equity.clone());
                new_equity
            }
        };
        equity
    }

    pub fn get_webhook(&mut self) -> &Option<String> {
        &mut self.webhook_url
    }

    pub fn set_webhook(&mut self, webhook_url: &str) {
        self.webhook_url = Some(webhook_url.into());
        self.save_config();
    }

    pub fn save_config(&mut self) {
        _ = self.write();
    }
}
// config commands
impl Config {
    pub fn new() -> Self {
        if let Ok(config) = Self::read() { return config }
        Config::new()
    }

    fn write(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let file = File::create("config")?;
        serde_json::to_writer(file, &self)?;
        Ok(())
    }

    fn read() -> Result<Self, Box<dyn std::error::Error>> {
        let file = match File::open("config") {
            Ok(file) => file,
            Err(_) => File::create("config").unwrap(),
        };
        let reader = BufReader::new(file);
        let resultant_config: Result<Self, serde_json::Error> = serde_json::from_reader(reader);
        match resultant_config {
            Ok(config) => Ok(config),
            Err(_) => Ok(Config::default()),
        }
    }
}
