/// Severity level of a fee spike relative to baseline.
#[derive(Debug, Clone, PartialEq)]
pub enum SpikeSeverity {
    /// 2–5× baseline
    Low,
    /// 5–10× baseline
    Medium,
    /// 10–50× baseline
    High,
    /// >50× baseline
    Critical,
}

/// A detected spike with its severity and duration.
#[derive(Debug, Clone, PartialEq)]
pub struct SpikeEvent {
    pub severity: SpikeSeverity,
    /// Number of consecutive ledgers the spike persisted.
    pub duration_ledgers: usize,
}

/// Classifies fee spikes in a time series.
pub struct SpikeClassifier;

impl SpikeClassifier {
    /// Classify a single fee against a baseline.
    /// Returns `None` if the fee is below the 2× spike threshold.
    pub fn classify(fee: u64, baseline: u64) -> Option<SpikeSeverity> {
        if baseline == 0 {
            return None;
        }
        let ratio = fee as f64 / baseline as f64;
        match ratio {
            r if r > 50.0 => Some(SpikeSeverity::Critical),
            r if r >= 10.0 => Some(SpikeSeverity::High),
            r if r >= 5.0 => Some(SpikeSeverity::Medium),
            r if r >= 2.0 => Some(SpikeSeverity::Low),
            _ => None,
        }
    }

    /// Detect all spike events in a fee sequence given a fixed baseline.
    /// Consecutive ledgers above the 2× threshold are grouped into one event.
    pub fn detect(fees: &[u64], baseline: u64) -> Vec<SpikeEvent> {
        let mut events = Vec::new();
        let mut i = 0;
        while i < fees.len() {
            if let Some(severity) = Self::classify(fees[i], baseline) {
                let start = i;
                // Advance while still a spike (any severity)
                while i < fees.len() && Self::classify(fees[i], baseline).is_some() {
                    i += 1;
                }
                // Severity of the event = max severity seen in the run
                let severity = fees[start..i]
                    .iter()
                    .filter_map(|&f| Self::classify(f, baseline))
                    .max_by_key(|s| match s {
                        SpikeSeverity::Low => 1,
                        SpikeSeverity::Medium => 2,
                        SpikeSeverity::High => 3,
                        SpikeSeverity::Critical => 4,
                    })
                    .unwrap_or(severity);
                events.push(SpikeEvent {
                    severity,
                    duration_ledgers: i - start,
                });
            } else {
                i += 1;
            }
        }
        events
    }
}
