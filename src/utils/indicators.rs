pub fn calculate_rsi(prices: &Vec<f64>, period: usize) -> Vec<f64> {
    let mut rsi_values = Vec::new();
    let mut avg_gain = 0.0;
    let mut avg_loss = 0.0;

    for i in 1..=period {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            avg_gain += change;
        } else {
            avg_loss -= change;
        }
    }

    avg_gain /= period as f64;
    avg_loss /= period as f64;

    let first_rsi = 100.0 - (100.0 / (1.0 + (avg_gain / avg_loss)));
    rsi_values.push(first_rsi);

    for i in (period + 1)..prices.len() {
        let change = prices[i] - prices[i - 1];
        let gain = if change > 0.0 { change } else { 0.0 };
        let loss = if change < 0.0 { -change } else { 0.0 };

        avg_gain = (avg_gain * (period as f64 - 1.0) + gain) / period as f64;
        avg_loss = (avg_loss * (period as f64 - 1.0) + loss) / period as f64;

        let rs = avg_gain / avg_loss;
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsi_values.push(rsi);
    }

    rsi_values
}
pub fn rsi(prices: &Vec<f64>, window: usize) -> Vec<f64> {
    if prices.len() < window {
        return vec![];
    }

    let mut gains = vec![0.0; prices.len()];
    let mut losses = vec![0.0; prices.len()];

    for i in 1..prices.len() {
        let change = prices[i] - prices[i - 1];
        if change > 0.0 {
            gains[i] = change;
        } else {
            losses[i] = -change;
        }
    }

    let mut rsi_values = vec![0.0; prices.len() - window];

    for i in 0..=(prices.len() - window - 1) {
        let avg_gain: f64 = gains[i + 1..i + 1 + window].iter().sum::<f64>() / window as f64;
        let avg_loss: f64 = losses[i + 1..i + 1 + window].iter().sum::<f64>() / window as f64;

        let rs = if avg_loss == 0.0 { f64::INFINITY } else { avg_gain / avg_loss };
        let rsi = 100.0 - (100.0 / (1.0 + rs));
        rsi_values[i] = rsi;
    }

    rsi_values
}
