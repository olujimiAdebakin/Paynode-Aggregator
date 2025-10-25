

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProposalModel {
    pub id: i32,
    pub proposal_id: Vec<u8>,
    pub order_id: Vec<u8>,
    pub provider: Vec<u8>,
    pub proposed_fee_bps: i32,
    pub status: String,  // PENDING, ACCEPTED, REJECTED, TIMED_OUT, EXECUTED
    pub created_at: DateTime<Utc>,
    pub deadline: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub executed_at: Option<DateTime<Utc>>,
    pub tx_hash: Option<Vec<u8>>,
}