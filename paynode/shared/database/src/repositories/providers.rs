
use sqlx::PgPool;
use crate::{
    error::Result,
    models::{ProviderIntentModel, ProviderReputationModel},
};

pub struct ProviderRepository {
    pool: PgPool,
}

impl ProviderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Upsert provider intent
    pub async fn upsert_intent(&self, intent: &ProviderIntentModel) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO provider_intents (
                provider, currency, available_amount, min_fee_bps, 
                max_fee_bps, commitment_window, is_active, expires_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (provider, currency) 
            DO UPDATE SET
                available_amount = $3,
                min_fee_bps = $4,
                max_fee_bps = $5,
                commitment_window = $6,
                is_active = $7,
                expires_at = $8,
                updated_at = NOW()
            "#,
            intent.provider,
            intent.currency,
            intent.available_amount,
            intent.min_fee_bps,
            intent.max_fee_bps,
            intent.commitment_window,
            intent.is_active,
            intent.expires_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get eligible providers for a currency and amount
    pub async fn get_eligible_providers(
        &self,
        currency: &str,
        min_amount: &str,
    ) -> Result<Vec<ProviderIntentModel>> {
        let providers = sqlx::query_as!(
            ProviderIntentModel,
            r#"
            SELECT 
                id, provider, currency, available_amount,
                min_fee_bps, max_fee_bps, commitment_window,
                is_active, expires_at, created_at, updated_at
            FROM provider_intents
            WHERE currency = $1
            AND available_amount >= $2
            AND is_active = true
            AND expires_at > NOW()
            ORDER BY min_fee_bps ASC
            "#,
            currency,
            min_amount
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(providers)
    }
    
    /// Get provider reputation
    pub async fn get_reputation(&self, provider: &[u8]) -> Result<Option<ProviderReputationModel>> {
        let reputation = sqlx::query_as!(
            ProviderReputationModel,
            r#"
            SELECT 
                provider, total_orders, successful_orders, failed_orders,
                no_shows, avg_settlement_time_seconds, total_volume, last_updated
            FROM provider_reputation
            WHERE provider = $1
            "#,
            provider
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(reputation)
    }
}