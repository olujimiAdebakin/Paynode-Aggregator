
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::enums::{OrderStatus, OrderTier, Currency};

/// Core order structure (domain model)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    /// Internal UUID for tracking
    pub id: Uuid,
    
    /// Blockchain order ID (bytes32 as hex string)
    pub order_id: String,
    
    /// User's wallet address
    pub user_address: String,
    
    /// Token contract address (e.g., USDC, USDT)
    pub token: String,
    
    /// Amount in smallest unit (wei for 18 decimals)
    pub amount: String,
    
    /// Address to send refunds if order fails
    pub refund_address: String,
    
    /// Integrator/dApp address
    pub integrator_address: String,
    
    /// Integrator fee in basis points (e.g., 50 = 0.5%)
    pub integrator_fee_bps: u64,
    
    /// Current order status
    pub status: OrderStatus,
    
    /// Order tier classification
    pub tier: OrderTier,
    
    /// Off-ramp currency (NGN, KES, etc.)
    pub currency: Currency,
    
    /// When order was created
    pub created_at: DateTime<Utc>,
    
    /// When order expires if not fulfilled
    pub expires_at: DateTime<Utc>,
    
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    
    /// Blockchain block number where order was created
    pub block_number: u64,
    
    /// Transaction hash of order creation
    pub tx_hash: String,
}

impl Order {
    /// Create a new order
    pub fn new(
        order_id: String,
        user_address: String,
        token: String,
        amount: String,
        refund_address: String,
        integrator_address: String,
        integrator_fee_bps: u64,
        currency: Currency,
        tier: OrderTier,
        expires_at: DateTime<Utc>,
        block_number: u64,
        tx_hash: String,
    ) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            order_id,
            user_address,
            token,
            amount,
            refund_address,
            integrator_address,
            integrator_fee_bps,
            status: OrderStatus::Pending,
            tier,
            currency,
            created_at: now,
            expires_at,
            updated_at: now,
            block_number,
            tx_hash,
        }
    }
    
    /// Check if order has expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
    
    
    /// Check if order can be refunded
    pub fn can_refund(&self) -> bool {
        self.is_expired() && 
        (self.status == OrderStatus::Pending || self.status == OrderStatus::Accepted)
    }
    
    /// Update order status
    pub fn update_status(&mut self, new_status: OrderStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }
}

/// Request to create an order
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub token: String,
    pub amount: String,
    pub currency: String,
    pub refund_address: String,
    pub integrator_address: String,
}

/// Order created event (from blockchain)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderCreatedEvent {
    pub order_id: String,
    pub user: String,
    pub token: String,
    pub amount: String,
    pub refund_address: String,
    pub integrator: String,
    pub block_number: u64,
    pub tx_hash: String,
    pub timestamp: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_order_creation() {
        let order = Order::new(
            "0x123...".to_string(),
            "0xuser...".to_string(),
            "0xusdc...".to_string(),
            "1000000000000000000".to_string(),
            "0xrefund...".to_string(),
            "0xintegrator...".to_string(),
            50,
            Currency::NGN,
            OrderTier::Alpha,
            Utc::now(),
            12345,
            "0xtx...".to_string(),
        );
        
        assert_eq!(order.status, OrderStatus::Pending);
        assert_eq!(order.tier, OrderTier::Alpha);
    }
    
    #[test]
    fn test_order_expiry() {
        let order = Order::new(
            "0x123...".to_string(),
            "0xuser...".to_string(),
            "0xusdc...".to_string(),
            "1000000000000000000".to_string(),
            "0xrefund...".to_string(),
            "0xintegrator...".to_string(),
            50,
            Currency::NGN,
            OrderTier::Alpha,
            Utc::now() - chrono::Duration::hours(1), // Already expired
            12345,
            "0xtx...".to_string(),
        );
        
        assert!(order.is_expired());
        assert!(order.can_refund());
    }
}