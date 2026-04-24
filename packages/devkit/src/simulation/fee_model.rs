//! Models for simulating Stellar transaction fee behaviour.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

/// Configuration for the fee simulation.
pub struct FeeModelConfig {
    /// Base fee in stroops.
    pub base_fee: u64,
    /// Probability [0.0, 1.0] that any given ledger is a spike.
    pub spike_probability: f64,
    /// Multiplier applied to base_fee during a spike.
    pub spike_multiplier: u64,
    /// Ledger close interval in seconds (used for timestamp spacing).
    pub ledger_interval_secs: u64,
    /// Optional RNG seed for reproducibility.
    pub seed: Option<u64>,
}

impl Default for FeeModelConfig {
    fn default() -> Self {
        Self {
            base_fee: 100,
            spike_probability: 0.05,
            spike_multiplier: 10,
            ledger_interval_secs: 5,
            seed: None,
        }
    }
}

/// A single simulated fee data point.
pub struct FeePoint {
    /// Simulated Unix timestamp (seconds).
    pub timestamp: u64,
    /// Fee in stroops for this ledger.
    pub fee: u64,
    /// Whether this ledger was a spike.
    pub is_spike: bool,
}

/// Models for simulating Stellar transaction fee behaviour.
pub struct FeeModel {
    config: FeeModelConfig,
    rng: SmallRng,
}

impl FeeModel {
    pub fn new(config: FeeModelConfig) -> Self {
        let rng = match config.seed {
            Some(s) => SmallRng::seed_from_u64(s),
            None => SmallRng::from_entropy(),
        };
        Self { config, rng }
    }

    /// Generate `count` fee points starting from `start_timestamp`.
    pub fn generate(&mut self, count: usize, start_timestamp: u64) -> Vec<FeePoint> {
        let mut points = Vec::with_capacity(count);
        for i in 0..count {
            let is_spike = self.rng.gen::<f64>() < self.config.spike_probability;
            let fee = if is_spike {
                self.config.base_fee * self.config.spike_multiplier
            } else {
                self.config.base_fee
            };
            points.push(FeePoint {
                timestamp: start_timestamp + (i as u64) * self.config.ledger_interval_secs,
                fee,
                is_spike,
            });
        }
        points
    }
}
