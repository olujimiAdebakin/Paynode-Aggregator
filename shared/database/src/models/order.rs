use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use shared_types::{OrderStatus, OrderTier};

/// Database representation of an Order
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OrderModel {
    pub id: i32,
    pub order_id: Vec<u8>,  // bytes32 from blockchain
    pub user_address: Vec<u8>,
    pub token: Vec<u8>,
    pub amount: String,  // Store as string to avoid precision issues
    pub refund_address: Vec<u8>,
    pub integrator: Vec<u8>,
    pub status: String,  // Will map to OrderStatus enum
    pub tier: Option<String>,  // Will map to OrderTier enum
    pub currency: Option<String>,
    pub block_number: i64,
    pub tx_hash: Vec<u8>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
}

impl OrderModel {
    /// Convert database model to domain type
    pub fn to_domain(&self) -> shared_types::Order {
        shared_types::Order {
            id: uuid::Uuid::new_v4(), // Generate for now
            order_id: format!("0x{}", hex::encode(&self.order_id)),
            user_address: format!("0x{}", hex::encode(&self.user_address)),
            token: format!("0x{}", hex::encode(&self.token)),
            amount: self.amount.clone(),
            currency: self.currency.clone().unwrap_or_default(),
            tier: self.parse_tier(),
            status: self.parse_status(),
            created_at: self.created_at,
            expires_at: self.expires_at.unwrap_or(self.created_at),
        }
    }
    
    fn parse_status(&self) -> OrderStatus {
        match self.status.as_str() {
            "PENDING" => OrderStatus::Pending,
            "ACCEPTED" => OrderStatus::Accepted,
            "FULFILLED" => OrderStatus::Fulfilled,
            "REFUNDED" => OrderStatus::Refunded,
            "EXPIRED" => OrderStatus::Expired,
            _ => OrderStatus::Pending,
        }
    }
    
    fn parse_tier(&self) -> OrderTier {
        match self.tier.as_deref() {
            Some("ALPHA") => OrderTier::Alpha,
            Some("BETA") => OrderTier::Beta,
            Some("DELTA") => OrderTier::Delta,
            Some("OMEGA") => OrderTier::Omega,
            Some("TITAN") => OrderTier::Titan,
            _ => OrderTier::Alpha,
        }
    }
}

/// Helper to convert hex string to bytes
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");
    hex::decode(hex).unwrap_or_default()
}