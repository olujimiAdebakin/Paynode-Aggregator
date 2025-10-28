# ✨ Paynode Shared Database Core

## Overview
This repository hosts the `shared-database` Rust crate, a foundational backend component for the Paynode ecosystem. It provides robust, type-safe, and asynchronous PostgreSQL database interactions, managing connection pools, schema migrations, and defining core data models and repository patterns for critical business entities like orders, providers, and proposals. Built with `sqlx` and `tokio`, it ensures reliable data persistence and retrieval for blockchain-integrated applications.

## Features
-   **Configurable Connection Pooling**: Efficient and resilient PostgreSQL connection management using `sqlx` and `tokio`, including connection health checks and automatic idle timeouts, optimized for services like Neon DB.
-   **Automated Schema Migrations**: Streamlined database schema evolution through `sqlx::migrate!`, ensuring consistency across development and production environments.
-   **Type-Safe ORM with SQLx**: Leverage Rust's strong type system with `sqlx` for compile-time checked SQL queries and direct mapping to `OrderModel`, `ProviderIntentModel`, `ProviderReputationModel`, and `ProposalModel` structs.
-   **Environment-Based Configuration**: Flexible database configuration loaded from environment variables, supporting `.env` files for local development.
-   **Comprehensive Error Handling**: Structured and ergonomic error management using a custom `DatabaseError` enum and `thiserror` crate, providing clear diagnostics for database operations.
-   **Domain Model Conversion**: Seamless transformation of raw database records into rich domain-specific types (e.g., converting `Vec<u8>` to hex strings, parsing `OrderStatus` enums), bridging the gap between database and application logic.
-   **Repository Pattern Implementation**: Clear separation of concerns with dedicated `OrderRepository`, `ProviderRepository`, and `ProposalRepository` structs, encapsulating data access logic.
-   **Integrator Fee Logic**: Includes business logic to dynamically fetch and apply integrator-specific fees, demonstrating support for customizable pricing models.

## Getting Started

### Installation
To get this project up and running locally, follow these steps:

1.  **Clone the Repository**:
    ```bash
    git clone https://github.com/olujimiAdebakin/paynode.git
    cd paynode/shared/database
    ```

2.  **Install Rust and Cargo**:
    If you don't have Rust installed, download it via `rustup`:
    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    source $HOME/.cargo/env
    ```

3.  **Install `sqlx-cli`**:
    This is required for managing database migrations.
    ```bash
    cargo install sqlx-cli --no-default-features --features "postgres, rustls"
    ```

4.  **Set up PostgreSQL Database**:
    Ensure you have a PostgreSQL database instance running and accessible. You can use Docker for a quick setup:
    ```bash
    docker run --name paynode-db -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=postgres -p 5432:5432 -d postgres:13
    ```

5.  **Create `.env` file**:
    Create a `.env` file in the root of the `shared/database` directory with your database connection string and other settings.

### Environment Variables
The following environment variables are required to configure the database connection:

*   `DATABASE_URL`: The full PostgreSQL connection string.
    **Example**: `postgresql://postgres:postgres@localhost:5432/paynode_dev`
*   `DATABASE_MAX_CONNECTIONS`: (Optional) Maximum number of connections in the pool. Defaults to `10`.
    **Example**: `DATABASE_MAX_CONNECTIONS=5`
*   `DATABASE_MIN_CONNECTIONS`: (Optional) Minimum number of connections in the pool. Defaults to `2`.
    **Example**: `DATABASE_MIN_CONNECTIONS=1`
*   `DATABASE_CONNECT_TIMEOUT_SECS`: (Optional) Timeout for establishing a new connection in seconds. Defaults to `30`.
    **Example**: `DATABASE_CONNECT_TIMEOUT_SECS=15`
*   `DATABASE_IDLE_TIMEOUT_SECS`: (Optional) How long a connection can be idle before being closed, in seconds. Defaults to `600`.
    **Example**: `DATABASE_IDLE_TIMEOUT_SECS=300`

### Usage
Here's how you can integrate and use the `shared-database` crate in your Rust application:

1.  **Add to your `Cargo.toml`**:
    First, ensure your project depends on `shared-database` and `tokio` (with full features for the runtime).
    ```toml
    # In your application's Cargo.toml (e.g., services/my-service/Cargo.toml)
    [dependencies]
    shared-database = { path = "../../shared/database" } # Adjust path as needed
    tokio = { version = "1.35", features = ["full"] }
    anyhow = "1.0"
    chrono = "0.4"
    ```

2.  **Initialize Database and Use Repositories**:
    A common pattern is to initialize the database at application startup and then create repository instances to perform operations.

    ```rust
    // In your application's main.rs or lib.rs
    use shared_database::{initialize_database, repositories::{OrderRepository, ProviderRepository}, models::OrderModel};
    use chrono::{DateTime, Utc};
    use anyhow::Result;

    #[tokio::main]
    async fn main() -> Result<()> {
        // Load environment variables from .env file (if not already loaded)
        dotenvy::dotenv().ok();

        // 1. Initialize the database pool, run migrations, and check connection
        let pool = initialize_database().await?;
        println!("✅ Database successfully initialized!");

        // 2. Create repository instances
        let order_repo = OrderRepository::new(pool.clone());
        let provider_repo = ProviderRepository::new(pool.clone());

        // --- Example: Creating an Order ---
        // (Note: `order_id`, `user_address`, `token`, etc., should come from your application's logic,
        // typically derived from blockchain events or user input, converted to Vec<u8>.)
        let dummy_order = OrderModel {
            id: 0, // Database will ignore this and generate a new ID
            order_id: vec![0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xab, 0xcd, 0xef], // Example 32-byte hash
            user_address: vec![0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44], // Example 20-byte address
            token: vec![0x55, 0x44, 0x33, 0x22, 0x11, 0x00, 0xFF, 0xEE, 0xDD, 0xCC, 0xBB, 0xAA, 0x99, 0x88, 0x77, 0x66, 0x55, 0x44, 0x33, 0x22],
            amount: "1000000000000000000".to_string(), // 1 unit in smallest denomination (e.g., 1 ETH in wei)
            refund_address: vec![0x1A, 0x2B, 0x3C, 0x4D, 0x5E, 0x6F, 0x7A, 0x8B, 0x9C, 0xAD, 0xBE, 0xCF, 0xDA, 0xEB, 0xFC, 0xED, 0xFE, 0xAF, 0xBF, 0xCF],
            integrator_address: vec![0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
            integrator_fee: vec![0x01], // Example integrator fee (raw bytes, context-dependent)
            status: "PENDING".to_string(),
            tier: Some("ALPHA".to_string()),
            currency: Some("USD".to_string()),
            block_number: 12345678,
            tx_hash: vec![0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF, 0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99],
            created_at: Utc::now(),
            expires_at: Some(Utc::now() + chrono::Duration::hours(24)), // Expires in 24 hours
            updated_at: Utc::now(),
        };

        let new_order_id = order_repo.create(&dummy_order).await?;
        println!("✨ Successfully created new order with internal ID: {}", new_order_id);

        // --- Example: Getting a pending order ---
        let pending_orders = order_repo.get_pending_orders().await?;
        if let Some(first_pending_order) = pending_orders.first() {
            println!("🔍 Found a pending order (internal ID {}): {:?}", first_pending_order.id, first_pending_order);
            // Example: Update its status
            order_repo.update_status(&first_pending_order.order_id, "ACCEPTED").await?;
            println!("✅ Updated status of order {} to ACCEPTED", hex::encode(&first_pending_order.order_id));
        } else {
            println!("No pending orders found.");
        }

        // --- Example: Upserting Provider Intent ---
        let dummy_provider_intent = shared_database::models::ProviderIntentModel {
            id: 0,
            provider: vec![0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80, 0x90, 0xA0, 0xB0, 0xC0, 0xD0, 0xE0, 0xF0, 0x11, 0x22, 0x33, 0x44, 0x55],
            currency: "USD".to_string(),
            available_amount: "5000.00".to_string(),
            min_fee_bps: 10, // 0.10%
            max_fee_bps: 50, // 0.50%
            commitment_window: 3600, // 1 hour
            is_active: true,
            expires_at: Utc::now() + chrono::Duration::days(7),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        provider_repo.upsert_intent(&dummy_provider_intent).await?;
        println!("✅ Successfully upserted provider intent.");

        Ok(())
    }
    ```

## Paynode Shared Database Core Library Documentation

### Crate Name
`shared-database`

### Modules and Key Structures

#### `shared_database::pool`
Provides functionality for managing PostgreSQL connection pools and migrations.

*   `fn create_pool_from_env() -> Result<PgPool>`
    **Purpose**: Initializes a PostgreSQL connection pool using configuration from environment variables (e.g., `DATABASE_URL`).
    **Returns**: A `PgPool` instance on success, or `DatabaseError` on failure.

*   `fn run_migrations(pool: &PgPool) -> Result<()>`
    **Purpose**: Executes pending SQL migrations defined in the `./migrations` directory.
    **Parameters**:
        - `pool`: `&PgPool` - A reference to the database connection pool.
    **Returns**: `()` on success, or `DatabaseError` on migration failure.

*   `fn check_connection(pool: &PgPool) -> Result<()>`
    **Purpose**: Performs a simple health check to verify database connectivity.
    **Parameters**:
        - `pool`: `&PgPool` - A reference to the database connection pool.
    **Returns**: `()` on success, or `DatabaseError` if the connection fails.

*   `fn initialize_database() -> Result<PgPool>`
    **Purpose**: High-level convenience function to create a pool, run migrations, and check connection in one go. This is the recommended entry point for services needing database access.
    **Returns**: A `PgPool` instance on success, or `DatabaseError` on failure.

#### `shared_database::repositories::OrderRepository`
Manages CRUD operations for `OrderModel` entities.

*   `fn new(pool: PgPool) -> Self`
    **Purpose**: Creates a new `OrderRepository` instance.
    **Parameters**:
        - `pool`: `PgPool` - The database connection pool to use.
    **Returns**: `OrderRepository`

*   `async fn create(&self, order: &OrderModel) -> Result<i32>`
    **Purpose**: Inserts a new order record into the database.
    **Request (OrderModel payload)**:
    ```rust
    pub struct OrderModel {
        // id: i32, // Auto-generated by DB
        pub order_id: Vec<u8>,
        pub user_address: Vec<u8>,
        pub token: Vec<u8>,
        pub amount: String,
        pub refund_address: Vec<u8>,
        pub integrator_address: Vec<u8>,
        pub integrator_fee: Vec<u8>, // Note: Named integrator_fees in repository's insert SQL
        pub status: String, // Maps to 'order_status' enum in DB (e.g., "PENDING", "ACCEPTED")
        pub tier: Option<String>, // Maps to 'order_tier' enum in DB (e.g., "ALPHA", "BETA")
        pub currency: Option<String>, // e.g., "USD"
        pub block_number: i64,
        pub tx_hash: Vec<u8>,
        pub created_at: DateTime<Utc>,
        pub expires_at: Option<DateTime<Utc>>,
        pub updated_at: DateTime<Utc>,
    }
    ```
    **Response**: `i32` (the auto-generated internal database ID of the new order).

*   `async fn get_by_order_id(&self, order_id: &[u8]) -> Result<OrderModel>`
    **Purpose**: Retrieves an order by its blockchain `order_id`.
    **Parameters**:
        - `order_id`: `&[u8]` - The unique blockchain order ID (32-byte hash).
    **Response**: `OrderModel` on success, or `DatabaseError::NotFound` if not found.

*   `async fn get_pending_orders(&self) -> Result<Vec<OrderModel>>`
    **Purpose**: Fetches all orders currently in 'PENDING' status.
    **Response**: `Vec<OrderModel>` - A list of pending orders.

*   `async fn update_status(&self, order_id: &[u8], new_status: &str) -> Result<()>`
    **Purpose**: Updates the status of an existing order.
    **Parameters**:
        - `order_id`: `&[u8]` - The blockchain order ID.
        - `new_status`: `&str` - The new status (e.g., "ACCEPTED", "FULFILLED", must match `order_status` enum variants).
    **Response**: `()` on success.

*   `async fn get_expired_orders(&self) -> Result<Vec<OrderModel>>`
    **Purpose**: Retrieves orders that are 'PENDING' and whose `expires_at` timestamp is in the past.
    **Response**: `Vec<OrderModel>` - A list of expired orders.

#### `shared_database::repositories::ProviderRepository`
Manages `ProviderIntentModel` and `ProviderReputationModel` entities.

*   `fn new(pool: PgPool) -> Self`
    **Purpose**: Creates a new `ProviderRepository` instance.
    **Parameters**:
        - `pool`: `PgPool` - The database connection pool to use.
    **Returns**: `ProviderRepository`

*   `async fn upsert_intent(&self, intent: &ProviderIntentModel) -> Result<()>`
    **Purpose**: Inserts or updates a provider's intent (their offering to the network). If an intent for the `(provider, currency)` pair already exists, it is updated.
    **Request (ProviderIntentModel payload)**:
    ```rust
    pub struct ProviderIntentModel {
        // id: i32, // Auto-generated by DB
        pub provider: Vec<u8>,           // Provider's wallet address
        pub currency: String,            // Target fiat currency (e.g., "USD", "EUR")
        pub available_amount: String,    // Amount available for off-ramping (as string)
        pub min_fee_bps: i32,            // Minimum fee in basis points (e.g., 10 for 0.10%)
        pub max_fee_bps: i32,            // Maximum fee in basis points
        pub commitment_window: i64,      // How long provider commits to intent (seconds)
        pub is_active: bool,             // Whether the intent is currently active
        pub expires_at: DateTime<Utc>,
        pub created_at: DateTime<Utc>,
        pub updated_at: DateTime<Utc>,
    }
    ```
    **Response**: `()` on success.

*   `async fn get_eligible_providers(&self, currency: &str, min_amount: &str) -> Result<Vec<ProviderIntentModel>>`
    **Purpose**: Retrieves active providers whose intents match the specified currency, available amount, and have not expired. Results are ordered by minimum fee.
    **Parameters**:
        - `currency`: `&str` - The target fiat currency (e.g., "USD").
        - `min_amount`: `&str` - The minimum amount the provider must be able to handle (as string).
    **Response**: `Vec<ProviderIntentModel>` - A list of eligible provider intents.

*   `async fn get_reputation(&self, provider: &[u8]) -> Result<Option<ProviderReputationModel>>`
    **Purpose**: Fetches the reputation data for a specific provider.
    **Parameters**:
        - `provider`: `&[u8]` - The provider's wallet address.
    **Response**: `Option<ProviderReputationModel>` - The provider's reputation, or `None` if no reputation data is found.
    ```rust
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
    ```

#### `shared_database::repositories::ProposalRepository`
Manages `ProposalModel` entities.

*   `fn new(pool: PgPool) -> Self`
    **Purpose**: Creates a new `ProposalRepository` instance.
    **Parameters**:
        - `pool`: `PgPool` - The database connection pool to use.
    **Returns**: `ProposalRepository`

*   `async fn create(&self, proposal: &ProposalModel) -> Result<i32>`
    **Purpose**: Inserts a new proposal record into the database.
    **Request (ProposalModel payload)**:
    ```rust
    pub struct ProposalModel {
        // id: i32, // Auto-generated by DB
        pub proposal_id: Vec<u8>,     // Unique blockchain proposal ID
        pub order_id: Vec<u8>,        // Associated blockchain order ID
        pub provider: Vec<u8>,        // Provider's wallet address
        pub proposed_fee_bps: i32,    // Proposed fee in basis points
        pub status: String,           // Maps to 'proposal_status' enum in DB (e.g., "PENDING", "ACCEPTED")
        pub created_at: DateTime<Utc>,
        pub deadline: DateTime<Utc>,
        pub accepted_at: Option<DateTime<Utc>>,
        pub executed_at: Option<DateTime<Utc>>,
        pub tx_hash: Option<Vec<u8>>,
    }
    ```
    **Response**: `i32` (the auto-generated internal database ID of the new proposal).

*   `async fn update_status(&self, proposal_id: &[u8], new_status: &str) -> Result<()>`
    **Purpose**: Updates the status of an existing proposal.
    **Parameters**:
        - `proposal_id`: `&[u8]` - The blockchain proposal ID.
        - `new_status`: `&str` - The new status (e.g., "ACCEPTED", "REJECTED", must match `proposal_status` enum variants).
    **Response**: `()` on success.

### Errors
This custom error enum encapsulates all possible database-related errors within the `shared-database` crate. All public functions return `Result<T, DatabaseError>`.

*   `ConnectionError(sqlx::Error)`: Indicates failure to establish or maintain a database connection.
*   `ConfigError(String)`: Signifies issues with database configuration, such as missing environment variables or malformed connection strings.
*   `MigrationError(String)`: Errors encountered during database schema migrations (e.g., failed SQL execution).
*   `NotFound(String)`: Occurs when a requested record could not be found in the database.
*   `DuplicateEntry(String)`: Raised when an attempt is made to insert a record that violates a unique constraint.
*   `InvalidData(String)`: Denotes that provided data is malformed or invalid for a specific database operation.
*   `TransactionError(String)`: A general error occurring within a database transaction context.

## Technologies Used
| Technology        | Description                                                          | Link                                                       |
| :---------------- | :------------------------------------------------------------------- | :--------------------------------------------------------- |
| **Rust**          | Primary programming language for high performance and reliability    | [rust-lang.org](https://www.rust-lang.org/)                |
| **SQLx**          | Asynchronous PostgreSQL driver with compile-time checked queries     | [github.com/launchbadge/sqlx](https://github.com/launchbadge/sqlx) |
| **Tokio**         | The leading asynchronous runtime for Rust                            | [tokio.rs](https://tokio.rs/)                              |
| **PostgreSQL**    | Robust, open-source object-relational database system                | [postgresql.org](https://www.postgresql.org/)              |
| **Chrono**        | Powerful date and time library for Rust                              | [docs.rs/chrono](https://docs.rs/chrono)                   |
| **Serde**         | Serialization and deserialization framework for Rust data structures | [serde.rs](https://serde.rs/)                              |
| **Thiserror**     | Derive macro for declarative error type definitions                  | [docs.rs/thiserror](https://docs.rs/thiserror)             |
| **Anyhow**        | Flexible concrete Error type, simplifying error handling             | [docs.rs/anyhow](https://docs.rs/anyhow)                   |
| **Tracing**       | Framework for instrumenting Rust programs to collect diagnostics     | [tokio.rs/tracing](https://tokio.rs/tracing)               |
| **Uuid**          | Library for generating and parsing Universally Unique Identifiers    | [docs.rs/uuid](https://docs.rs/uuid)                       |
| **Hex**           | Efficient hexadecimal encoding and decoding utilities                | [docs.rs/hex](https://docs.rs/hex)                         |
| **Rust_decimal**  | Arbitrary-precision decimal arithmetic                               | [docs.rs/rust_decimal](https://docs.rs/rust_decimal)       |
| **Dotenvy**       | Library for loading environment variables from `.env` files          | [docs.rs/dotenvy](https://docs.rs/dotenvy)                 |
| **Shared-Types**  | Local internal crate providing shared domain models and enums        | (Local Crate within Paynode workspace)                     |

## Contributing
We welcome contributions to the `shared-database` crate! If you're interested in improving this project, please follow these guidelines:

*   **Fork the Repository**: 🍴 Start by forking the `paynode` repository to your GitHub account.
*   **Create a New Branch**: 🌿 Create a new branch for your feature or bug fix: `git checkout -b feature/your-feature-name` or `bugfix/issue-description`.
*   **Write Clear Code**: ✨ Ensure your code adheres to Rust's best practices and includes comprehensive tests.
*   **Commit Messages**: 📝 Use descriptive commit messages following conventional commits (e.g., `feat: Add new order status`).
*   **Open a Pull Request**: 🚀 Submit a pull request to the `main` branch of the `paynode` repository, detailing your changes and the problem they solve.

## License
This project is licensed under the MIT License.

## Author Info
*   **Your Name**: [Your LinkedIn](https://www.linkedin.com/in/yourprofile) | [Your Twitter](https://twitter.com/yourhandle)

## Badges
![Rust](https://img.shields.io/badge/Rust-black?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-1A1B23?style=for-the-badge&logo=tokio&logoColor=white)
[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)