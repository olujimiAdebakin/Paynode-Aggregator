# Paynode Shared Database

## Overview
A robust and asynchronous Rust library designed to manage PostgreSQL database interactions for the Paynode ecosystem. This module provides a structured approach to data persistence, offering defined models and efficient repositories for core entities such as financial orders, provider proposals, and provider intentions and reputations. It leverages `sqlx` for compile-time checked SQL queries and `tokio` for asynchronous operations, ensuring high performance and data integrity. 💾✨

## Features
-   **PostgreSQL Integration**: Seamless connectivity and interaction with PostgreSQL databases using the `sqlx` asynchronous ORM.
-   **Connection Pooling**: Efficiently manages database connections with `PgPoolOptions` for optimal resource utilization and performance.
-   **Database Migrations**: Automated schema evolution with `sqlx::migrate!` to ensure database consistency across environments.
-   **Structured Data Models**: Defines `OrderModel`, `ProposalModel`, `ProviderIntentModel`, and `ProviderReputationModel` for clear data representation.
-   **Type-Safe Repositories**: Provides dedicated repositories (`OrderRepository`, `ProposalRepository`, `ProviderRepository`) for CRUD operations with compile-time checked queries.
-   **Robust Error Handling**: Comprehensive error management using `thiserror` for clear and actionable database-related errors.
-   **Asynchronous Operations**: Fully asynchronous design built on `tokio` for non-blocking database interactions.

## Getting Started

To get the Paynode Shared Database module up and running locally, follow these steps.

### Prerequisites
*   [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
*   [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) (Rust's package manager, installed with Rust)
*   [PostgreSQL](https://www.postgresql.org/download/) (version 12+)

### Installation
1.  **Clone the Repository**
    ```bash
    git clone https://github.com/olujimiAdebakin/paynode.git
    cd paynode/shared/database
    ```
2.  **Build the Project**
    ```bash
    cargo build
    ```
3.  **Database Setup (Local PostgreSQL)**
    For local development, it is highly recommended to use Docker Compose to spin up a PostgreSQL instance. Create a `docker-compose.yml` in your project root (or a suitable location) if you don't already have one, for example:
    ```yaml
    version: '3.8'
    services:
      db:
        image: postgres:14
        restart: always
        environment:
          POSTGRES_USER: postgres
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: paynode
        ports:
          - "5432:5432"
        volumes:
          - db_data:/var/lib/postgresql/data
    
    volumes:
      db_data:
    ```
    Then, from the directory containing your `docker-compose.yml`, run:
    ```bash
    docker-compose up -d
    ```

### Environment Variables
The module relies on environment variables for database configuration. Ensure the following variable is set:

*   `DATABASE_URL`: The connection string for your PostgreSQL database.
    *   **Example**: `postgresql://postgres:postgres@localhost:5432/paynode`

    You can set this in your shell or integrate it using a crate like `dotenv` in your main application.

## Usage

This module is intended to be used as a library in other Rust services or applications within the Paynode ecosystem.

### Adding as a Dependency
To use `shared-database` in your Rust project, add it to your `Cargo.toml`. Assuming `paynode/shared/database` is a sibling directory to your consuming project's `Cargo.toml`:

```toml
[dependencies]
shared-database = { path = "../shared/database" }
# Ensure these features match what is used in shared-database's Cargo.toml
sqlx = { version = "0.7", features = ["runtime-tokio-native-tls", "postgres", "uuid", "chrono", "json"] } 
tokio = { version = "1.35", features = ["full"] }
# If `shared-types` is also a local dependency
shared-types = { path = "../shared/types" } 
```

### Initializing the Database
The `initialize_database` helper function can be used to set up the connection pool and run migrations automatically upon application startup.

```rust
use shared_database::{
    initialize_database, 
    repositories::orders::OrderRepository, 
    models::order::{OrderModel, hex_to_bytes}, 
    error::Result
};
use chrono::{Utc, Duration};
use shared_types::{OrderStatus, OrderTier}; // Assuming shared_types is correctly set up

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing for better logging (optional but highly recommended)
    tracing_subscriber::fmt::init();

    // Establish database connection and run pending migrations
    let pool = initialize_database().await?;
    tracing::info!("Database initialized and migrations run successfully.");

    // Instantiate a repository
    let order_repo = OrderRepository::new(pool.clone());

    // Example: Creating a new order entry
    let new_order = OrderModel {
        id: 0, // This field is typically auto-incremented by the DB, 
              // but required for struct instantiation.
        order_id: hex_to_bytes("0x1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b"),
        user_address: hex_to_bytes("0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef"),
        token: hex_to_bytes("0x0000000000000000000000000000000000000000"), // Example token
        amount: "100.00".to_string(),
        refund_address: hex_to_bytes("0x1234567890abcdef1234567890abcdef12345678"),
        integrator: hex_to_bytes("0xabc123abc123abc123abc123abc123abc123abc1"),
        status: OrderStatus::Pending.to_string(),
        tier: Some(OrderTier::Alpha.to_string()),
        currency: Some("USDC".to_string()),
        block_number: 12345678,
        tx_hash: hex_to_bytes("0xfeedfacedeadbeefdeadbeefdeadbeefdeadbeef"),
        created_at: Utc::now(),
        expires_at: Some(Utc::now() + Duration::hours(24)),
        updated_at: Utc::now(),
    };

    let inserted_id = order_repo.create(&new_order).await?;
    tracing::info!("Successfully created order with database ID: {}", inserted_id);

    // Example: Fetching an order by its blockchain ID
    let fetched_order = order_repo.get_by_order_id(&new_order.order_id).await?;
    tracing::info!("Fetched order details: {:?}", fetched_order.to_domain());

    // Example: Updating an order's status
    order_repo.update_status(&new_order.order_id, OrderStatus::Accepted.to_string().as_str()).await?;
    tracing::info!("Order status updated to ACCEPTED.");

    Ok(())
}
```

### Database Error Handling
The module provides a custom `DatabaseError` enum for structured and explicit error handling within the Paynode context:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DatabaseError {
    #[error("Database connection error: {0}")]
    ConnectionError(#[from] sqlx::Error),
    
    #[error("Migration error: {0}")]
    MigrationError(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
    
    #[error("Duplicate entry: {0}")]
    DuplicateEntry(String),
    
    #[error("Invalid data: {0}")]
    InvalidData(String),
    
    #[error("Transaction error: {0}")]
    TransactionError(String),
}

pub type Result<T> = std::result::Result<T, DatabaseError>;
```
All public functions within the `shared-database` module's API return `Result<T>`, which simplifies robust error propagation and handling in consuming applications.

## Technologies Used

| Technology    | Description                                                 |
| :------------ | :---------------------------------------------------------- |
| **Rust**      | Modern systems programming language focusing on safety and performance. |
| **SQLx**      | Asynchronous, compile-time checked SQL queries for Rust, preventing common database errors at build time. |
| **Tokio**     | The de-facto asynchronous runtime for Rust, enabling non-blocking I/O. |
| **PostgreSQL**| A powerful, open-source object-relational database system, known for reliability, feature robustness, and performance. |
| **Anyhow**    | A flexible concrete Error type for idiomatic error handling in Rust applications. |
| **Thiserror** | A derive macro for the standard `Error` trait, simplifying custom error type creation. |
| **Tracing**   | A framework for instrumenting Rust programs to collect scoped, structured, and async-aware diagnostics. |
| **Serde**     | A powerful serialization/deserialization framework for efficiently working with various data formats. |
| **Chrono**    | A comprehensive date and time library for Rust, used for timestamp management. |
| **UUID**      | A library for generating and parsing Universally Unique Identifiers. |
| **Hex**       | Provides efficient hexadecimal encoding and decoding for byte arrays. |

## Contributing
We welcome contributions to the Paynode Shared Database module! To contribute, please follow these guidelines:

*   **Fork the repository** and create your branch from `main`.
*   **Ensure your code adheres** to existing coding styles and conventions, using `cargo fmt` and `cargo clippy`.
*   **Write clear, concise commit messages** that explain the purpose of your changes.
*   **Submit a pull request** with a detailed description of your changes, referencing any relevant issues.
*   **Run tests** (`cargo test`) to ensure your changes haven't introduced regressions.
*   **Add new tests** for new features or bug fixes to maintain high code coverage.

## License
This project is licensed under the MIT License.

## Author Info
**Olujimi Adebakin**
*   LinkedIn: [Your LinkedIn Profile](https://linkedin.com/in/yourusername)
*   Twitter: [@yourtwitterhandle](https://twitter.com/yourtwitterhandle)

## Badges
![Rust](https://img.shields.io/badge/Rust-F74B00?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)
![SQLx](https://img.shields.io/badge/SQLx-black?style=for-the-badge&logo=sqlx&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-black?style=for-the-badge&logo=tokio&logoColor=white)
![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)
[![Build Status](https://img.shields.io/badge/Build-Passing-brightgreen)](https://github.com/olujimiAdebakin/paynode/actions/workflows/rust.yml)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)