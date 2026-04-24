//! Generates synthetic network load profiles for simulation.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct NetworkLoadConfig {
    /// Minimum transactions per ledger.
    pub min_tx: u64,
    /// Maximum transactions per ledger.
    pub max_tx: u64,
    /// Optional RNG seed for reproducibility.
    pub seed: Option<u64>,
}

impl Default for NetworkLoadConfig {
    fn default() -> Self {
        Self {
            min_tx: 10,
            max_tx: 1000,
            seed: None,
        }
    }
}

/// Generates synthetic network load profiles for simulation.
pub struct NetworkLoad {
    config: NetworkLoadConfig,
    rng: SmallRng,
}

impl NetworkLoad {
    pub fn new(config: NetworkLoadConfig) -> Self {
        let rng = match config.seed {
            Some(s) => SmallRng::seed_from_u64(s),
            None => SmallRng::from_entropy(),
        };
        Self { config, rng }
    }

    /// Generate `count` transaction-count samples.
    pub fn generate(&mut self, count: usize) -> Vec<u64> {
        (0..count)
            .map(|_| self.rng.gen_range(self.config.min_tx..=self.config.max_tx))
            .collect()
    }
}
