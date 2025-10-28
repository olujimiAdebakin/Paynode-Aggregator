
use sqlx::PgPool;
use crate::{error::Result, models::ProposalModel};

pub struct ProposalRepository {
    pool: PgPool,
}

impl ProposalRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Create a new proposal
    pub async fn create(&self, proposal: &ProposalModel) -> Result<i32> {
        let record = sqlx::query!(
            r#"
            INSERT INTO proposals (
                proposal_id, order_id, provider, proposed_fee_bps,
                status, created_at, deadline
            ) VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id
            "#,
            proposal.proposal_id,
            proposal.order_id,
            proposal.provider,
            proposal.proposed_fee_bps,
            proposal.status,
            proposal.created_at,
            proposal.deadline
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(record.id)
    }
    
    /// Update proposal status
   pub async fn update_status(&self, proposal_id: &[u8], new_status: &str) -> Result<()> {
    sqlx::query!(
        r#"
        UPDATE proposals
        SET status = $1::proposal_status
        WHERE proposal_id = $2
        "#,
        new_status,
        proposal_id
    )
    .execute(&self.pool)
    .await?;
    
    Ok(())
}


}