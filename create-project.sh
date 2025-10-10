#!/bin/bash

# Payflow Project Generator
PROJECT_NAME="paynode"

echo "Creating $PROJECT_NAME project structure..."

# Create root directory
mkdir -p $PROJECT_NAME
cd $PROJECT_NAME

# Create main directories
mkdir -p services/{api-gateway,order-service,ai-router,provider-service,balance-service,settlement-service,analytics-service,blockchain-indexer}/src
mkdir -p shared/{types,database,messaging,utils}/src
mkdir -p scripts k8s/{deployments,services,configmaps,secrets} tests/{integration,load} docs

# Create Cargo workspace
cat > Cargo.toml << 'EOF'
[workspace]
members = [
    "services/api-gateway",
    "services/order-service",
    "services/ai-router",
    "services/provider-service",
    "services/balance-service",
    "services/settlement-service",
    "services/analytics-service",
    "services/blockchain-indexer",
    "shared/types",
    "shared/database",
    "shared/messaging",
    "shared/utils",
]

resolver = "2"

[workspace.dependencies]
tokio = { version = "1.35", features = ["full"] }
axum = "0.7"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono", "decimal"] }
redis = { version = "0.24", features = ["tokio-comp", "connection-manager"] }
uuid = { version = "1.6", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
rust_decimal = "1.33"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
anyhow = "1.0"
thiserror = "1.0"
dotenv = "0.15"
async-nats = "0.33"
reqwest = { version = "0.11", features = ["json"] }
EOF

# API Gateway
cat > services/api-gateway/Cargo.toml << 'EOF'
[package]
name = "api-gateway"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
axum = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
tracing = { workspace = true }
tracing-subscriber = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }
shared-types = { path = "../../shared/types" }
shared-messaging = { path = "../../shared/messaging" }
EOF

cat > services/api-gateway/src/main.rs << 'EOF'
use axum::{routing::get, Router};
use std::net::SocketAddr;
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    let app = Router::new()
        .route("/health", get(health_check));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    info!("API Gateway listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
EOF

# Order Service
cat > services/order-service/Cargo.toml << 'EOF'
[package]
name = "order-service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
axum = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
rust_decimal = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
shared-types = { path = "../../shared/types" }
shared-database = { path = "../../shared/database" }
shared-messaging = { path = "../../shared/messaging" }
EOF

cat > services/order-service/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Order Service starting...");

    Ok(())
}
EOF

# AI Router Service
cat > services/ai-router/Cargo.toml << 'EOF'
[package]
name = "ai-router"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
reqwest = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
shared-types = { path = "../../shared/types" }
shared-messaging = { path = "../../shared/messaging" }
EOF

cat > services/ai-router/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("AI Router Service starting...");

    Ok(())
}
EOF

# Provider Service
cat > services/provider-service/Cargo.toml << 'EOF'
[package]
name = "provider-service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
axum = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
shared-types = { path = "../../shared/types" }
shared-database = { path = "../../shared/database" }
EOF

cat > services/provider-service/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Provider Service starting...");

    Ok(())
}
EOF

# Balance Service
cat > services/balance-service/Cargo.toml << 'EOF'
[package]
name = "balance-service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
redis = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
rust_decimal = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
shared-types = { path = "../../shared/types" }
shared-database = { path = "../../shared/database" }
EOF

cat > services/balance-service/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Balance Service starting...");

    Ok(())
}
EOF

# Settlement Service
cat > services/settlement-service/Cargo.toml << 'EOF'
[package]
name = "settlement-service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
ethers = "2.0"
shared-types = { path = "../../shared/types" }
EOF

cat > services/settlement-service/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Settlement Service starting...");

    Ok(())
}
EOF

# Analytics Service
cat > services/analytics-service/Cargo.toml << 'EOF'
[package]
name = "analytics-service"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
sqlx = { workspace = true }
serde = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
shared-types = { path = "../../shared/types" }
shared-database = { path = "../../shared/database" }
EOF

cat > services/analytics-service/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Analytics Service starting...");

    Ok(())
}
EOF

# Blockchain Indexer
cat > services/blockchain-indexer/Cargo.toml << 'EOF'
[package]
name = "blockchain-indexer"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { workspace = true }
serde = { workspace = true }
tracing = { workspace = true }
anyhow = { workspace = true }
dotenv = { workspace = true }
ethers = "2.0"
shared-types = { path = "../../shared/types" }
shared-messaging = { path = "../../shared/messaging" }
EOF

cat > services/blockchain-indexer/src/main.rs << 'EOF'
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    info!("Blockchain Indexer starting...");

    Ok(())
}
EOF

# Shared: Types
cat > shared/types/Cargo.toml << 'EOF'
[package]
name = "shared-types"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { workspace = true }
uuid = { workspace = true }
chrono = { workspace = true }
rust_decimal = { workspace = true }
EOF

cat > shared/types/src/lib.rs << 'EOF'
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub status: OrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Assigned,
    Processing,
    Fulfilled,
    Validated,
    Settled,
    Cancelled,
    Refunded,
}
EOF

# Shared: Database
cat > shared/database/Cargo.toml << 'EOF'
[package]
name = "shared-database"
version = "0.1.0"
edition = "2021"

[dependencies]
sqlx = { workspace = true }
anyhow = { workspace = true }
EOF

cat > shared/database/src/lib.rs << 'EOF'
use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;
    
    Ok(pool)
}
EOF

# Shared: Messaging
cat > shared/messaging/Cargo.toml << 'EOF'
[package]
name = "shared-messaging"
version = "0.1.0"
edition = "2021"

[dependencies]
async-nats = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
anyhow = { workspace = true }
EOF

cat > shared/messaging/src/lib.rs << 'EOF'
use async_nats::Client;
use anyhow::Result;

pub async fn connect_nats(url: &str) -> Result<Client> {
    let client = async_nats::connect(url).await?;
    Ok(client)
}
EOF

# Shared: Utils
cat > shared/utils/Cargo.toml << 'EOF'
[package]
name = "shared-utils"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = { workspace = true }
EOF

cat > shared/utils/src/lib.rs << 'EOF'
pub fn validate_currency(currency: &str) -> bool {
    matches!(currency, "NGN" | "GHS" | "KES" | "USD" | "EUR")
}
EOF

# Docker Compose
cat > docker-compose.yml << 'EOF'
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: payflow-postgres
    ports:
      - "5432:5432"
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: payflow
    volumes:
      - postgres_data:/var/lib/postgresql/data

  redis:
    image: redis:7-alpine
    container_name: payflow-redis
    ports:
      - "6379:6379"

  nats:
    image: nats:latest
    container_name: payflow-nats
    ports:
      - "4222:4222"
      - "8222:8222"

volumes:
  postgres_data:
EOF

# Environment file
cat > .env.example << 'EOF'
# Database
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/payflow
REDIS_URL=redis://localhost:6379
NATS_URL=nats://localhost:4222

# AI
GEMINI_API_KEY=your_api_key_here

# Services
API_GATEWAY_PORT=8000
ORDER_SERVICE_PORT=8001
AI_ROUTER_PORT=8002

# Sharding
SHARD_ID=1
SUPPORTED_CURRENCIES=NGN,GHS,KES
EOF

# README
cat > README.md << 'EOF'
# Payflow - AI-Driven Payment Aggregator

## Quick Start
```bash
# 1. Install dependencies
cargo build

# 2. Start databases
docker-compose up -d

# 3. Copy environment file
cp .env.example .env

# 4. Run API Gateway
cargo run --bin api-gateway

# 5. Run tests
cargo test