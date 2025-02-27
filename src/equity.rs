use std::fmt::{self, Display};

use crate::utils::{indicators::calculate_rsi, structs::HistoricalData};
#[derive(Debug, Clone)]
pub struct Equity {
    pub historical_data: crate::utils::structs::HistoricalData,
    pub ticker: String,
    rsi_undervalued: f64,
    rsi_overvalued: f64,
}
pub struct InvalidData;

impl fmt::Display for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error has occured, Inalid data has been used.")
    }
}
impl fmt::Debug for InvalidData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{ file: {}, line: {} }}", file!(), line!()) // programmer-facing output
    }
}

impl Equity {
    pub fn new(ticker: String, historical_data: HistoricalData) -> Self {
        Equity {
            ticker,
            historical_data,
            rsi_undervalued: 25.0,
            rsi_overvalued: 70.0,
        }
    }

    pub fn current_rsi(&mut self, window: usize) -> Result<f64, InvalidData> {
        Ok(*self.rsi_values(window)?.last().unwrap())
    }
    pub fn current_default_rsi(&mut self) -> Result<f64, InvalidData> {
        self.current_rsi(14)
    }
    pub fn rsi_values(&mut self, window: usize) -> Result<Vec<f64>, InvalidData> {
        calculate_rsi(&self.historical_data.close, window)
    }
    pub fn is_undervalued(&mut self) -> Result<bool, InvalidData> {
        Ok(self.current_rsi(14).unwrap() < self.rsi_undervalued)
    }
    pub fn is_overvalued(&mut self) -> Result<bool, InvalidData> {
        Ok(self.current_rsi(14).unwrap() > self.rsi_overvalued)
    }
    pub fn current_price(&mut self) -> Result<f64, InvalidData> {
        let closing = &self.historical_data.close;
        Ok(closing[closing.len()-1])
    }
    pub fn price_change(&mut self) -> Result<f64, InvalidData> {
        let closing = &self.historical_data.close;
        Ok(closing[closing.len()-1] - closing[closing.len()-2])
    }
}

impl Display for Equity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let closing_price = self.historical_data.close[self.historical_data.close.len()-1];
        write!(f, "{} at {}", self.ticker, closing_price)
    }
}
