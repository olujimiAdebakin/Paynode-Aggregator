
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Provider reputation metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderReputation {
    /// Provider address
    pub provider: String,
    
    /// Total orders attempted
    pub total_orders: u64,
    
    /// Successfully completed orders
    pub successful_orders: u64,
    
    /// Failed orders
    pub failed_orders: u64,
    
    /// Times provider didn't respond to proposal
    pub no_shows: u64,
    
    /// Average settlement time in seconds
    pub avg_settlement_time_seconds: u64,
    
    /// Total volume processed (as string to avoid overflow)
    pub total_volume: String,
    
    /// Last reputation update
    pub last_updated: DateTime<Utc>,
}

impl ProviderReputation {
    /// Create default reputation for new provider
    pub fn new(provider: String) -> Self {
        Self {
            provider,
            total_orders: 0,
            successful_orders: 0,
            failed_orders: 0,
            no_shows: 0,
            avg_settlement_time_seconds: 0,
            total_volume: "0".to_string(),
            last_updated: Utc::now(),
        }
    }
    
    /// Calculate success rate (0.0 to 1.0)
    pub fn success_rate(&self) -> f64 {
        if self.total_orders == 0 {
            return 0.5; // Neutral for new providers
        }
        self.successful_orders as f64 / self.total_orders as f64
    }
    
    /// Calculate reliability score (0.0 to 1.0)
    pub fn reliability_score(&self) -> f64 {
        if self.total_orders == 0 {
            return 0.5;
        }
        let show_rate = 1.0 - (self.no_shows as f64 / self.total_orders as f64);
        show_rate
    }
    
    /// Update after successful settlement
    pub fn record_success(&mut self, settlement_time_seconds: u64, amount: &str) {
        self.total_orders += 1;
        self.successful_orders += 1;
        
        // Update average settlement time
        let total_time = self.avg_settlement_time_seconds * (self.successful_orders - 1);
        self.avg_settlement_time_seconds = (total_time + settlement_time_seconds) / self.successful_orders;
        
        // Update volume
        let current_volume: u128 = self.total_volume.parse().unwrap_or(0);
        let new_amount: u128 = amount.parse().unwrap_or(0);
        self.total_volume = (current_volume + new_amount).to_string();
        
        self.last_updated = Utc::now();
    }
    
    /// Update after failed settlement
    pub fn record_failure(&mut self) {
        self.total_orders += 1;
        self.failed_orders += 1;
        self.last_updated = Utc::now();
    }
    
    /// Update after no-show
    pub fn record_no_show(&mut self) {
        self.total_orders += 1;
        self.no_shows += 1;
        self.last_updated = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_reputation_calculation() {
        let mut reputation = ProviderReputation::new("0xprovider...".to_string());
        
        // Record some activity
        reputation.record_success(120, "1000000000000000000000");
        reputation.record_success(90, "2000000000000000000000");
        reputation.record_failure();
        reputation.record_no_show();
        
        assert_eq!(reputation.total_orders, 4);
        assert_eq!(reputation.successful_orders, 2);
        assert_eq!(reputation.success_rate(), 0.5);
        assert_eq!(reputation.avg_settlement_time_seconds, 105); // (120 + 90) / 2
    }
}