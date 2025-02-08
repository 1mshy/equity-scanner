use serde_json::Value;

pub struct HistoricalData {
    pub metadata: Value,
    pub timestamp: Vec<u64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub open: Vec<f64>,
    pub close: Vec<f64>,
    pub volume: Vec<u64>,
}

impl HistoricalData {
    pub fn new(yahoo_raw: &Value) -> Self {
        let result = yahoo_raw
            .get("chart")
            .and_then(|c| c.get("result"))
            .and_then(|r| r.get(0))
            .unwrap();

        let metadata = result.get("meta").cloned().unwrap_or_default();

        let quotes = result
            .get("indicators")
            .and_then(|i| i.get("quote"))
            .and_then(|q| q.get(0))
            .unwrap();

        let timestamp = result
            .get("timestamp")
            .and_then(|t| t.as_array())
            .map(|arr| arr.iter().map(|v| v.as_u64().unwrap_or(0)).collect())
            .unwrap_or_default();

        let close = quotes
            .get("close")
            .and_then(|c| c.as_array())
            .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
            .unwrap_or_default();

        let open = quotes
            .get("open")
            .and_then(|o| o.as_array())
            .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
            .unwrap_or_default();

        let high = quotes
            .get("high")
            .and_then(|h| h.as_array())
            .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
            .unwrap_or_default();

        let low = quotes
            .get("low")
            .and_then(|l| l.as_array())
            .map(|arr| arr.iter().map(|v| v.as_f64().unwrap_or(0.0)).collect())
            .unwrap_or_default();

        let volume = quotes
            .get("volume")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().map(|v| v.as_u64().unwrap_or(0)).collect())
            .unwrap_or_default();

        Self {
            metadata,
            timestamp,
            high,
            low,
            open,
            close,
            volume,
        }
    }
}
