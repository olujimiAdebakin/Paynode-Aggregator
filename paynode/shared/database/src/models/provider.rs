
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProviderIntentModel {
    pub id: i32,
    pub provider: Vec<u8>,
    pub currency: String,
    pub available_amount: String,
    pub min_fee_bps: i32,
    pub max_fee_bps: i32,
    pub commitment_window: i64,
    pub is_active: bool,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProviderReputationModel {
    pub provider: Vec<u8>,
    pub total_orders: i64,
    pub successful_orders: i64,
    pub failed_orders: i64,
    pub no_shows: i64,
    pub avg_settlement_time_seconds: i64,
    pub total_volume: String,
    pub last_updated: DateTime<Utc>,
}