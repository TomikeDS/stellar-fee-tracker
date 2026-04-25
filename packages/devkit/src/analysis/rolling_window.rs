/// Maintains a rolling window of fee observations.
pub struct RollingWindow {
    window: usize,
    buf: std::collections::VecDeque<f64>,
}

impl RollingWindow {
    pub fn new(window: usize) -> Self {
        assert!(window > 0, "window size must be > 0");
        Self { window, buf: std::collections::VecDeque::with_capacity(window) }
    }

    /// Push a new fee value and return the current SMA if the window is full.
    pub fn push(&mut self, fee: f64) -> Option<f64> {
        if self.buf.len() == self.window {
            self.buf.pop_front();
        }
        self.buf.push_back(fee);
        if self.buf.len() == self.window {
            Some(self.buf.iter().sum::<f64>() / self.window as f64)
        } else {
            None
        }
    }

    /// Compute SMA over a complete slice with the configured window size.
    /// Returns one value per position once the window is full.
    pub fn sma(fees: &[f64], window: usize) -> Vec<f64> {
        assert!(window > 0, "window size must be > 0");
        if fees.len() < window {
            return vec![];
        }
        let mut result = Vec::with_capacity(fees.len() - window + 1);
        let mut sum: f64 = fees[..window].iter().sum();
        result.push(sum / window as f64);
        for i in window..fees.len() {
            sum += fees[i] - fees[i - window];
            result.push(sum / window as f64);
        }
        result
    }
}
