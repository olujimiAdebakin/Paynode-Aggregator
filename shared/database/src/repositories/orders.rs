use sqlx::PgPool;
use crate::{error::Result, models::OrderModel};

pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
    
    /// Insert a new order
    pub async fn create(&self, order: &OrderModel) -> Result<i32> {
        let record = sqlx::query!(
            r#"
            INSERT INTO orders (
                order_id, user_address, token, amount, 
                refund_address, integrator_address, integrator_fees, status, tier, 
                currency, block_number, tx_hash, created_at, expires_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8::order_status, $9::order_tier, $10, $11, $12, $13, $14)
            RETURNING id
            "#,
            order.order_id,
            order.user_address,
            order.token,
            order.amount,
            order.refund_address,
            order.integrator_address,
            order.integrator_fees,
            order.status.as_str(),      // I Convert enum to string
            order.tier.as_str(),         // I Convert enum to string
            order.currency,
            order.block_number,
            order.tx_hash,
            order.created_at,
            order.expires_at
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(record.id)
    }
    
    /// Get order by blockchain order_id (bytes32)
    pub async fn get_by_order_id(&self, order_id: &[u8]) -> Result<OrderModel> {
        let order = sqlx::query_as!(
            OrderModel,
            r#"
            SELECT 
                id, order_id, user_address, token, amount,
                refund_address, integrator_address, integrator_fees,
                status as "status: OrderStatus",
                tier as "tier: OrderTier",
                currency,
                block_number, tx_hash, created_at, expires_at, updated_at
            FROM orders
            WHERE order_id = $1
            "#,
            order_id
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(order)
    }
    
    /// Get all pending orders
    pub async fn get_pending_orders(&self) -> Result<Vec<OrderModel>> {
        let orders = sqlx::query_as!(
            OrderModel,
            r#"
            SELECT 
                id, order_id, user_address, token, amount,
                refund_address, integrator_address, integrator_fees,
                status as "status: OrderStatus",
                tier as "tier: OrderTier",
                currency,
                block_number, tx_hash, created_at, expires_at, updated_at
            FROM orders
            WHERE status = 'PENDING'
            ORDER BY created_at DESC
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(orders)
    }
    
    /// Update order status
    pub async fn update_status(&self, order_id: &[u8], new_status: &str) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE orders
            SET status = $1::order_status, updated_at = NOW()
            WHERE order_id = $2
            "#,
            .bind(new_status),
            .bind(dorder_id)
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    /// Get expired orders
    pub async fn get_expired_orders(&self) -> Result<Vec<OrderModel>> {
        let orders = sqlx::query_as!(
            OrderModel,
            r#"
            SELECT 
                id, order_id, user_address, token, amount,
                refund_address, integrator_address, integrator_fees,
                status as "status: OrderStatus",
                tier as "tier: OrderTier",
                currency,
                block_number, tx_hash, created_at, expires_at, updated_at
            FROM orders
            WHERE status = 'PENDING'
            AND expires_at < NOW()
            "#
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(orders)
    }
}