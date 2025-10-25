# Paynode API

## Overview
This repository houses the core shared libraries for the Paynode backend ecosystem, developed in Rust. It provides foundational components for building robust and scalable financial services 🚀, encompassing strongly typed data models for transactions, asynchronous messaging capabilities via NATS, and essential utility functions.

## Features
- `shared-types`: Defines canonical data structures, such as `Order` and `OrderStatus`, ensuring consistent and type-safe data representation across all Paynode services.
- `shared-messaging`: Provides a reliable client for NATS, enabling efficient and scalable asynchronous inter-service communication within the Paynode architecture.
- `shared-utils`: Offers a collection of common helper functions, including stringent currency validation, to enhance data integrity and streamline development.

## Getting Started
### Installation
To set up the project locally, follow these steps:

```bash
# Clone the repository
git clone https://github.com/olujimiAdebakin/paynode.git
cd paynode

# Build individual shared crates (optional, for verification)

# For shared-types:
cd shared/types
cargo build
cargo test
cd ../../.. # Navigate back to the root directory

# For shared-messaging:
cd shared/messaging
cargo build
cargo test
cd ../../.. # Navigate back to the root directory

# For shared-utils:
cd shared/utils
cargo build
cargo test
cd ../../.. # Navigate back to the root directory
```

### Environment Variables
The following environment variables are required for configuring the shared libraries:

- `NATS_URL`: The URL for connecting to the NATS message broker. This is primarily used by the `shared-messaging` crate.
  - Example: `NATS_URL=nats://127.0.0.1:4222`

## API Documentation
This section outlines how the shared types defined in this project would typically be used within an overarching Paynode API. While this repository provides the building blocks, actual API endpoints would be implemented in dedicated service repositories.

### Base URL
`/api/v1`

### Endpoints

#### POST /api/v1/orders
Creates a new financial order in the system. The server will assign a unique ID, set the initial status to `pending`, and record the creation timestamp.

**Request**:
```json
{
  "amount": "100.50",
  "currency": "NGN"
}
```

**Response**:
```json
{
  "id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "amount": "100.50",
  "currency": "NGN",
  "status": "pending",
  "created_at": "2023-10-27T10:30:00Z"
}
```

**Errors**:
- `400 Bad Request`: Returned if the `amount` is invalid (e.g., non-numeric, negative) or if the `currency` is not supported.
- `500 Internal Server Error`: Indicates an unexpected error occurred on the server while processing the request.

#### GET /api/v1/orders/{id}
Retrieves the details of a specific order using its unique identifier.

**Request**:
(No request body required)

**Response**:
```json
{
  "id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
  "amount": "100.50",
  "currency": "NGN",
  "status": "pending",
  "created_at": "2023-10-27T10:30:00Z"
}
```

**Errors**:
- `404 Not Found`: No order could be found corresponding to the provided `id`.
- `400 Bad Request`: The provided `id` is not in a valid UUID format.

## Usage
To integrate these shared libraries into your own Rust project, add them as dependencies in your project's `Cargo.toml`.

```toml
# In your project's Cargo.toml
[dependencies]
shared-types = { path = "../paynode/shared/types" } # Adjust path relative to your project
shared-messaging = { path = "../paynode/shared/messaging" } # Adjust path relative to your project
shared-utils = { path = "../paynode/shared/utils" } # Adjust path relative to your project
```

### Example: Using `shared-types` for Order Management
```rust
use shared_types::{Order, OrderStatus};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    let new_order = Order {
        id: Uuid::new_v4(),
        amount: Decimal::from_str("250.75").unwrap(),
        currency: "USD".to_string(),
        status: OrderStatus::Pending,
        created_at: Utc::now(),
    };

    println!("New Order: {:?}", new_order);

    // You can then serialize this order for storage or transmission
    let serialized_order = serde_json::to_string(&new_order).unwrap();
    println!("Serialized Order: {}", serialized_order);
}
```

### Example: Using `shared-messaging` to Connect to NATS
```rust
use shared_messaging::connect_nats;
use anyhow::Result;

#[tokio::main] // Requires the Tokio runtime
async fn main() -> Result<()> {
    let nats_url = std::env::var("NATS_URL")
        .unwrap_or_else(|_| "nats://127.0.0.1:4222".to_string());

    let client = connect_nats(&nats_url).await?;
    println!("Successfully connected to NATS at {}", nats_url);

    // Example: Publish a simple message to a subject
    client.publish("my.application.events", "Hello NATS from Paynode!".into()).await?;
    println!("Published message to 'my.application.events'");

    Ok(())
}
```

### Example: Using `shared-utils` for Currency Validation
```rust
use shared_utils::validate_currency;

fn main() {
    println!("Is NGN a valid currency? {}", validate_currency("NGN")); // true
    println!("Is JPY a valid currency? {}", validate_currency("JPY")); // false
    println!("Is USD a valid currency? {}", validate_currency("USD")); // true
}
```

## Technologies
The Paynode shared libraries are built using the following key technologies and Rust crates:

| Technology     | Description                                                          | Link                                                       |
| :------------- | :------------------------------------------------------------------- | :--------------------------------------------------------- |
| **Rust**       | The robust and performant systems programming language.              | [rust-lang.org](https://www.rust-lang.org/)                |
| **async-nats** | An asynchronous NATS client library for Rust, enabling message-driven architectures. | [crates.io](https://crates.io/crates/async-nats)           |
| **serde**      | A powerful framework for efficiently serializing and deserializing Rust data structures. | [serde.rs](https://serde.rs/)                              |
| **uuid**       | Provides functionality for generating and parsing Universally Unique Identifiers. | [crates.io](https://crates.io/crates/uuid)                 |
| **chrono**     | A comprehensive date and time library for Rust.                      | [crates.io](https://crates.io/crates/chrono)               |
| **rust_decimal** | Facilitates arbitrary-precision decimal arithmetic, crucial for financial calculations. | [crates.io](https://crates.io/crates/rust_decimal)         |
| **anyhow**     | A flexible and convenient error handling library for Rust applications. | [crates.io](https://crates.io/crates/anyhow)               |

## Contributing
We welcome contributions to the Paynode shared libraries! To contribute, please follow these guidelines:

*   **Fork the repository**: Start by forking the `paynode` repository to your GitHub account.
*   **Create a new branch**: For each feature, enhancement, or bug fix, create a dedicated branch (e.g., `feature/add-new-payment-type`, `bugfix/improve-currency-validation`).
*   **Write clear, idiomatic Rust code**: Ensure your code is well-structured, readable, and adheres to Rust best practices and coding standards.
*   **Add tests**: Include comprehensive unit and integration tests for new features or bug fixes to maintain robust code quality and prevent regressions.
*   **Use descriptive commit messages**: Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) specification for clear and concise commit history.
*   **Submit a Pull Request**: Open a pull request against the `main` branch, providing a detailed description of your changes, the problem it solves, and any relevant context.

## License
This project is licensed under the MIT License. Please refer to the `LICENSE` file in the root of the repository for full details. *(Note: As no explicit `LICENSE` file was provided in the project context, this is a placeholder for future inclusion.)*

## Author Info
Developed by a passionate Rust developer committed to building scalable and reliable backend infrastructure.

*   **LinkedIn**: [Your LinkedIn Profile](https://linkedin.com/in/your_username)
*   **Twitter**: [Your Twitter Profile](https://twitter.com/your_username)
*   **Personal Website**: [Your Website](https://your-website.com)

## Badges
[![Rust](https://img.shields.io/badge/Rust-shared%20crates-orange?style=flat&logo=rust)](https://www.rust-lang.org/)
[![async-nats](https://img.shields.io/badge/NATS-Messaging-blue?style=flat&logo=nats)](https://nats.io/)
[![Serde](https://img.shields.io/badge/Serde-Serialization-green?style=flat&logo=serde)](https://serde.rs/)
[![UUID](https://img.shields.io/badge/UUID-Identifier-lightgrey?style=flat&logo=uuid)](https://crates.io/crates/uuid)
[![Chrono](https://img.shields.io/badge/Chrono-Datetime-yellow?style=flat&logo=chrono)](https://crates.io/crates/chrono)
[![Rust Decimal](https://img.shields.io/badge/Decimal-Precision-purple?style=flat)](https://crates.io/crates/rust_decimal)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)
