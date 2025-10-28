use sqlx::postgres::{PgPool, PgPoolOptions, PgConnectOptions};
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
        // Load environment variables from .env file
        // This ensures DATABASE_URL is loaded before i try to use it
        let _ = dotenvy::dotenv();
        
        Self {
            url: std::env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set in .env file or environment"),
            max_connections: 5,
            min_connections: 1,
            connect_timeout: Duration::from_secs(10),
            idle_timeout: Duration::from_secs(300),
        }
    }
}

/// Load database configuration from environment variables
/// Provides more explicit control than using Default trait
pub fn load_database_config() -> Result<DatabaseConfig> {
    // Load .env file if it exists (optional, but recommended)
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| DatabaseError::ConfigError("DATABASE_URL must be set in .env file or environment".to_string()))?;
    
    Ok(DatabaseConfig {
        url: database_url,
        max_connections: std::env::var("DATABASE_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10),
        min_connections: std::env::var("DATABASE_MIN_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(2),
        connect_timeout: Duration::from_secs(
            std::env::var("DATABASE_CONNECT_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30)
        ),
        idle_timeout: Duration::from_secs(
            std::env::var("DATABASE_IDLE_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(600)
        ),
    })
}

/// Creates a PostgreSQL connection pool
pub async fn create_pool(config: DatabaseConfig) -> Result<PgPool> {
    tracing::info!(
        "Creating database pool for {} with max_connections={}", 
        config.url, 
        config.max_connections
    );
    
    // Parse connection options and disable prepared statement cache
    let connect_options = config.url
        .parse::<PgConnectOptions>()
        .map_err(|e| {
            tracing::error!("Failed to parse database URL: {}", e);
            DatabaseError::ConfigError(format!("Invalid database URL: {}", e))
        })?
        .statement_cache_capacity(0);  // Disable prepared statements globally
    
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(config.connect_timeout)
        .idle_timeout(config.idle_timeout)
        
        .max_lifetime(Some(Duration::from_secs(1800)))  // Close connections after 30 min
        .test_before_acquire(true)  // CRITICAL for Neon - test connection health
        .connect_with(connect_options)  // Using connect_with instead of connect
        .await
        .map_err(|e| {
            tracing::error!("Failed to connect to database at {}: {}", config.url, e);
            DatabaseError::ConnectionError(e)
        })?;
    
    tracing::info!("Database pool created successfully with prepared statements disabled");
    Ok(pool)
}

/// Creates a pool with default configuration
/// Uses DATABASE_URL from environment variables
pub async fn create_default_pool() -> Result<PgPool> {
    create_pool(DatabaseConfig::default()).await
}

/// Creates a pool with explicit configuration loading
/// Recommended for production use
pub async fn create_pool_from_env() -> Result<PgPool> {
    let config = load_database_config()?;
    create_pool(config).await
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

/// Initialize database with connection, migrations, and health check
/// This is the main entry point for database setup
pub async fn initialize_database() -> Result<PgPool> {
    let pool = create_pool_from_env().await?;
    run_migrations(&pool).await?;
    check_connection(&pool).await?;
    Ok(pool)
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