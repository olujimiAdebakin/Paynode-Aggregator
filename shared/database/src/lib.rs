pub mod error;
pub mod pool;
pub mod models;
pub mod repositories;

// Re-export commonly used items
pub use error::{DatabaseError, Result};
pub use pool::{create_pool, create_default_pool, create_pool_from_env, run_migrations, check_connection,load_database_config,  DatabaseConfig};
pub use repositories::{OrderRepository, ProviderRepository, ProposalRepository};

// Helper function to initialize database for a service
pub async fn initialize_database() -> Result<sqlx::PgPool> {
    let pool = create_pool_from_env().await?;
    run_migrations(&pool).await?; 
    check_connection(&pool).await?;
    Ok(pool)
}