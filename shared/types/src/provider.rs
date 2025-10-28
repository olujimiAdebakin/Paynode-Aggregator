
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::enums::Currency;

/// Provider intent to offer liquidity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderIntent {
    /// Provider's wallet address
    pub provider: String,
    
    /// Currency they're offering (NGN, KES, etc.)
    pub currency: Currency,
    
    /// Available liquidity amount
    pub available_amount: String,
    
    /// Minimum fee they'll accept (basis points)
    pub min_fee_bps: u64,
    
    /// Maximum fee they'll accept (basis points)
    pub max_fee_bps: u64,
    
    /// How long they commit to accept proposals (seconds)
    pub commitment_window_seconds: u64,
    
    /// Is intent currently active
    pub is_active: bool,
    
    /// When intent was registered
    pub registered_at: DateTime<Utc>,
    
    /// When intent expires
    pub expires_at: DateTime<Utc>,
}

impl ProviderIntent {
    /// Check if intent is still valid
    pub fn is_valid(&self) -> bool {
        self.is_active && Utc::now() < self.expires_at
    }
    
    /// Check if provider can handle order amount
    pub fn can_handle_amount(&self, amount: &str) -> bool {
        let available: u128 = self.available_amount.parse().unwrap_or(0);
        let requested: u128 = amount.parse().unwrap_or(u128::MAX);
        available >= requested
    }
    
    /// Check if fee is within provider's range
    pub fn accepts_fee(&self, fee_bps: u64) -> bool {
        fee_bps >= self.min_fee_bps && fee_bps <= self.max_fee_bps
    }
}

/// Provider registration request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterProviderRequest {
    pub currency: String,
    pub available_amount: String,
    pub min_fee_bps: u64,
    pub max_fee_bps: u64,
    pub commitment_window_seconds: u64,
}

/// Provider intent updated event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderIntentEvent {
    pub provider: String,
    pub currency: String,
    pub available_amount: String,
    pub min_fee_bps: u64,
    pub max_fee_bps: u64,
    pub commitment_window: u64,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    
    #[test]
    fn test_provider_intent_validity() {
        let intent = ProviderIntent {
            provider: "0xprovider...".to_string(),
            currency: Currency::NGN,
            available_amount: "5000000000000000000000".to_string(),
            min_fee_bps: 200,
            max_fee_bps: 500,
            commitment_window_seconds: 300,
            is_active: true,
            registered_at: Utc::now(),
            expires_at: Utc::now() + Duration::hours(1),
        };
        
        assert!(intent.is_valid());
        assert!(intent.can_handle_amount("1000000000000000000000"));
        assert!(intent.accepts_fee(300));
        assert!(!intent.accepts_fee(100)); // Too low
        assert!(!intent.accepts_fee(600)); // Too high
    }
}