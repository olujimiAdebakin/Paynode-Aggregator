use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use shared_types::{OrderStatus, OrderTier, Currency};

/// Database representation of an Order
/// Maps directly to the PostgreSQL orders table structure
/// This struct represents the raw database record before conversion to domain model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct OrderModel {
    /// Auto-incrementing primary key for internal database reference
    /// Note: This is different from the blockchain order_id
    pub id: i32,
    /// Unique order identifier from blockchain (bytes32 hash)
    /// This is the primary business identifier for orders
    pub order_id: Vec<u8>,
    /// User's wallet address that created the order (20-byte Ethereum address)
    pub user_address: Vec<u8>,
    /// Token contract address being swapped (20-byte Ethereum address)
    pub token: Vec<u8>,
    /// Amount in smallest token units (wei for 18 decimals)
    /// Stored as string to avoid precision issues with large numbers
    pub amount: String,
    /// Address to refund tokens if order fails or expires (20-byte Ethereum address)
    pub refund_address: Vec<u8>,
    /// Integrator/dApp address that initiated the order (20-byte Ethereum address)
    /// Used to lookup integrator-specific fee configuration
    pub integrator_address: Vec<u8>,

    pub integrator_fee:  Vec<u8>,
    /// Current order status as string (maps to OrderStatus enum)
    /// Stored as string for PostgreSQL ENUM compatibility
    pub status: String,
    /// Order tier classification (maps to OrderTier enum)
    /// Optional to handle legacy orders or data migration scenarios
    pub tier: Option<String>,
    /// Target fiat currency for off-ramp operation
    /// Optional to support orders without specific currency requirement
    pub currency: Option<String>,
    /// Blockchain block number when order was created
    /// Used for blockchain synchronization and event replay
    pub block_number: i64,
    /// Transaction hash of order creation on blockchain
    /// Provides cryptographic proof of order creation
    pub tx_hash: Vec<u8>,
    /// Timestamp when order was created in the system
    /// Used for ordering and expiration calculations
    pub created_at: DateTime<Utc>,
    /// Timestamp when order expires if not fulfilled
    /// Optional to support orders without explicit expiry
    pub expires_at: Option<DateTime<Utc>>,
    /// Timestamp when order was last updated
    /// Used for change tracking and synchronization
    pub updated_at: DateTime<Utc>,
}

impl OrderModel {
    /// Converts database model to domain type for business logic
    /// Transforms database-specific types (BYTEA, ENUM strings) to domain types
    /// Performs necessary type conversions and data enrichment
    /// 
    /// # Arguments
    /// * `db_pool` - Database connection pool for fetching related data
    /// 
    /// # Returns
    /// * `Result<shared_types::Order, sqlx::Error>` - Domain order or database error
    pub async fn to_domain(&self, db_pool: &sqlx::PgPool) -> Result<shared_types::Order, sqlx::Error> {
        // Fetch integrator-specific fee from database
        // Integrators set their own fees via PayNode dashboard
        let integrator_fee_bps = self.get_integrator_fee_bps(db_pool).await?;
        
        Ok(shared_types::Order {
            // TODO: Implement proper UUID mapping from blockchain order_id
            // Currently generating new UUID as placeholder
            id: uuid::Uuid::new_v4(),
            
            // Convert bytes to hex string with 0x prefix for display and API responses
            order_id: format!("0x{}", hex::encode(&self.order_id)),
            user_address: format!("0x{}", hex::encode(&self.user_address)),
            token: format!("0x{}", hex::encode(&self.token)),
            refund_address: format!("0x{}", hex::encode(&self.refund_address)),
            integrator_address: format!("0x{}", hex::encode(&self.integrator_address)),
            tx_hash: format!("0x{}", hex::encode(&self.tx_hash)),
            
            // Amount remains as string to preserve precision across serialization
            amount: self.amount.clone(),
            
            // Parse database strings to strongly-typed enums with safe fallbacks
            currency: Currency::from_str(self.currency.as_deref().unwrap_or_default()),
            tier: self.parse_tier(),
            status: self.parse_status(),
            
            // Use integrator-configured fee from database
            // This fee is set by the integrator in their PayNode dashboard
            integrator_fee_bps,
            
            // Copy timestamp values directly (no conversion needed)
            updated_at: self.updated_at,
            created_at: self.created_at,
            
            // Convert database BIGINT (i64) to domain u64 for blockchain compatibility
            block_number: self.block_number as u64,
            
            // Use explicit expiry time or fallback to creation time for orders without expiry
            expires_at: self.expires_at.unwrap_or(self.created_at),
        })
    }
    
    /// Fetches integrator-specific fee from database
    /// Integrators configure their own fees via the PayNode dashboard
    /// This enables flexible pricing strategies per integrator
    /// 
    /// # Arguments
    /// * `db_pool` - Database connection pool for querying integrator_fees table
    /// 
    /// # Returns
    /// * `Result<u64, sqlx::Error>` - Fee in basis points or database error
    async fn get_integrator_fee_bps(&self, db_pool: &sqlx::PgPool) -> Result<u64, sqlx::Error> {
        // Query integrator_fees table using integrator_address as primary key
        // This query is optimized by the PRIMARY KEY index on integrator_address
        let result = sqlx::query!(
            "SELECT fee_bps FROM integrator_fees WHERE integrator_address = $1",
            &self.integrator_address
        )
        .fetch_optional(db_pool)
        .await?;
        
        // Return configured fee or default (50 bps = 0.50%) if not configured
        // This ensures orders can proceed even if integrator hasn't set a custom fee
        Ok(result.map(|r| r.fee_bps as u64).unwrap_or(50))
    }
    
    /// Parses database status string into OrderStatus enum
    /// Provides safe fallback to Pending status for unknown values
    /// Ensures system stability even with unexpected database values
    /// 
    /// # Returns
    /// * `OrderStatus` - Parsed status enum, defaults to Pending
    fn parse_status(&self) -> OrderStatus {
        match self.status.as_str() {
            "PENDING" => OrderStatus::Pending,
            "ACCEPTED" => OrderStatus::Accepted,
            "FULFILLED" => OrderStatus::Fulfilled,
            "REFUNDED" => OrderStatus::Refunded,
            "EXPIRED" => OrderStatus::Expired,
            _ => {
                // Log unexpected status values for monitoring
                // Default to Pending to maintain system operation
                OrderStatus::Pending
            }
        }
    }
    
    /// Parses database tier string into OrderTier enum
    /// Provides safe fallback to Alpha tier for unknown values
    /// Handles optional tier field for data flexibility
    /// 
    /// # Returns
    /// * `OrderTier` - Parsed tier enum, defaults to Alpha
    fn parse_tier(&self) -> OrderTier {
        match self.tier.as_deref() {
            Some("ALPHA") => OrderTier::Alpha,
            Some("BETA") => OrderTier::Beta,
            Some("DELTA") => OrderTier::Delta,
            Some("OMEGA") => OrderTier::Omega,
            Some("TITAN") => OrderTier::Titan,
            _ => {
                // Default to Alpha tier for unknown or missing tier values
                // This ensures order processing can continue normally
                OrderTier::Alpha
            }
        }
    }
}

/// Utility function to convert hex string to bytes
/// Commonly used for converting user input or API data to database format
/// Handles optional 0x prefix and provides safe error handling
/// 
/// # Arguments
/// * `hex` - Hex string potentially with 0x prefix
/// 
/// # Returns
/// * `Vec<u8>` - Decoded bytes, empty vector on decoding failure
pub fn hex_to_bytes(hex: &str) -> Vec<u8> {
    let hex = hex.trim_start_matches("0x");
    // Use unwrap_or_default to return empty vector rather than panicking on invalid hex
    // This ensures the system remains stable even with malformed input
    hex::decode(hex).unwrap_or_default()
}