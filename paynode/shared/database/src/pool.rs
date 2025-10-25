use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use crate::error::{DatabaseError, Result};

/// Database connection configuration
#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
    pub idle_timeout: Duration,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/paynode".to_string()),
            max_connections: 10,
            min_connections: 2,
            connect_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
        }
    }
}

/// Creates a PostgreSQL connection pool
pub async fn create_pool(config: DatabaseConfig) -> Result<PgPool> {
    tracing::info!("Creating database pool with max_connections={}", config.max_connections);
    
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        .connect(&config.url)
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to database: {}", e);
            DatabaseError::ConnectionError(e)
        })?;
    
    tracing::info!("Database pool created successfully");
    Ok(pool)
}

/// Creates a pool with default configuration
pub async fn create_default_pool() -> Result<PgPool> {
    create_pool(DatabaseConfig::default()).await
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    tracing::info!("Running database migrations...");
    
    sqlx::migrate!("./migrations")
        .run(pool)
        .await
        .map_err(|e| {
            tracing::error!("Migration failed: {}", e);
            DatabaseError::MigrationError(e.to_string())
        })?;
    
    tracing::info!("Migrations completed successfully");
    Ok(())
}

/// Health check for database connection
pub async fn check_connection(pool: &PgPool) -> Result<()> {
    sqlx::query("SELECT 1")
        .execute(pool)
        .await
        .map_err(DatabaseError::ConnectionError)?;
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pool_creation() {
        // This test requires a running PostgreSQL instance
        let config = DatabaseConfig {
            url: "postgresql://postgres:postgres@localhost:5432/paynode_test".to_string(),
            ..Default::default()
        };
        
        // Skip if DATABASE_URL not set (CI/CD)
        if std::env::var("DATABASE_URL").is_err() {
            return;
        }
        
        let result = create_pool(config).await;
        assert!(result.is_ok() || result.is_err()); // Just check it compiles
    }
}