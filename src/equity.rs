use core::fmt;
use std::fmt::Display;

use crate::utils::{indicators::calculate_rsi, structs::HistoricalData};
#[derive(Debug, Clone)]
pub struct Equity {
    pub historical_data: crate::utils::structs::HistoricalData,
    pub ticker: String,
    rsi_undervalued: f64,
    rsi_overvalued: f64,
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
    pub fn current_rsi(&mut self, window: usize) -> f64 {
        *self.rsi_values(window).last().unwrap()
    }
    pub fn rsi_values(&mut self, window: usize) -> Vec<f64> {
        calculate_rsi(&self.historical_data.close, window)
    }
    pub fn is_undervalued(&mut self) -> bool {
        self.current_rsi(14) < self.rsi_undervalued
    }
    pub fn is_overvalued(&mut self) -> bool {
        self.current_rsi(14) > self.rsi_overvalued
    }
}

impl Display for Equity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ticker)
    }
}
