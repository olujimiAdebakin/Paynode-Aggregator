# **Paynode Shared Core**

A robust collection of foundational Rust libraries designed to empower the Paynode ecosystem with secure, scalable, and efficient decentralized payment processing. This repository encapsulates critical domain logic, data structures, and communication utilities, enabling seamless interaction across various Paynode services. 🚀

## ✨ Features

*   **Decentralized Order Management:** Structured data types for defining, tracking, and managing off-chain settlement orders, including status transitions and tier classifications.
*   **Provider Settlement Proposals:** Comprehensive models for managing provider-submitted settlement proposals, their lifecycle, and acceptance/rejection flows.
*   **Fiat Payment Proofs & Requests:** Standardized formats for capturing and verifying fiat payment proofs from providers and generating payment requests for users.
*   **Dynamic Provider Reputation Tracking:** Algorithms and data structures to measure and update provider reliability, success rates, and average settlement times.
*   **Asynchronous NATS Messaging:** Utilities for establishing reliable, high-performance asynchronous communication channels between Paynode services using NATS.
*   **Robust Error Handling:** A dedicated error type (`TypesError`) using `thiserror` for consistent and clear error propagation across shared components.
*   **Cross-service Data Serialization:** Leverages `serde` for efficient serialization and deserialization of all core data types, ensuring interoperability.
*   **Blockchain Integration Readiness:** Includes optional `ethers` features and helper functions for interacting with Ethereum addresses and cryptographic hashes.

## 🚀 Getting Started

These libraries are intended to be consumed by other Rust services within the Paynode architecture. Follow these steps to set up the project locally.

### Installation

To clone the repository and prepare the shared modules:

```bash
git clone https://github.com/olujimiAdebakin/paynode.git
cd paynode
```

To build and test the shared components:

```bash
cargo build --workspace
cargo test --workspace
```

### Usage

To integrate these shared components into another Rust service, add the desired crate(s) to your project's `Cargo.toml`:

```toml
[dependencies]
shared-types = { path = "./shared/types" } # Or adjust path if in a different workspace
shared-messaging = { path = "./shared/messaging" }
shared-utils = { path = "./shared/utils" }
```

You can then import and use the types and functions in your application logic:

**Example: Using Shared Types**

```rust
use shared_types::{Order, OrderStatus, OrderTier, Currency};
use chrono::Utc;
use uuid::Uuid;

fn create_new_order_example() -> Order {
    let expires_at = Utc::now() + chrono::Duration::hours(24);
    Order::new(
        "0x123abc...".to_string(), // Blockchain order ID
        "0xUserAddress...".to_string(),
        "0xTokenAddress...".to_string(),
        "1000000000000000000000".to_string(), // 1000 tokens (assuming 18 decimals)
        "0xRefundAddress...".to_string(),
        "0xIntegratorAddress...".to_string(),
        50, // 0.5% fee
        Currency::NGN,
        OrderTier::Delta,
        expires_at,
        123456, // Block number
        "0xTransactionHash...".to_string(),
    )
}

// In your application logic:
let mut my_order = create_new_order_example();
println!("Initial order status: {:?}", my_order.status);
my_order.update_status(OrderStatus::Accepted);
println!("Updated order status: {:?}", my_order.status);
```

**Example: Using Shared Messaging**

```rust
use shared_messaging::connect_nats;
use anyhow::Result;

async fn setup_nats_connection() -> Result<()> {
    let nats_client = connect_nats("nats://127.0.0.1:4222").await?;
    println!("Successfully connected to NATS!");
    // Use nats_client for publishing/subscribing
    Ok(())
}

// In your main function or async block:
// #[tokio::main]
// async fn main() -> Result<()> {
//     setup_nats_connection().await
// }
```

## 🛠️ Technologies Used

| Technology       | Description                                                                                             |
| :--------------- | :------------------------------------------------------------------------------------------------------ |
| **Rust**         | The primary language for building performant and reliable systems.                                      |
| **`async-nats`** | Asynchronous client for the NATS messaging system, enabling real-time communication.                    |
| **`serde`**      | A powerful serialization framework for efficiently converting Rust data structures to various formats.    |
| **`sqlx`**       | An asynchronous, compile-time checked ORM for PostgreSQL, ensuring type-safe database interactions.     |
| **`chrono`**     | Date and time library for handling timestamps and durations.                                            |
| **`uuid`**       | Library for generating and working with Universally Unique Identifiers.                                 |
| **`thiserror`**  | Provides convenient derive macros for creating well-structured and readable error types.                |
| **`anyhow`**     | A flexible trait object based error handling utility, simplifying error propagation.                    |
| **`ethers`**     | (Optional) Comprehensive Ethereum wallet and blockchain interaction library.                            |
| **`hex`**        | Encoding and decoding of hexadecimal strings.                                                           |

## 🤝 Contributing

We welcome contributions to the Paynode Shared Core! To contribute:

1.  🍴 Fork the repository.
2.  🌿 Create a new branch (`git checkout -b feature/your-feature-name`).
3.  📝 Make your changes and ensure tests pass (`cargo test`).
4.  💬 Commit your changes (`git commit -m 'feat: Add new feature'`).
5.  ⬆️ Push to the branch (`git push origin feature/your-feature-name`).
6.  🚀 Open a Pull Request.

Please ensure your code adheres to existing style guidelines and includes appropriate test coverage.

## ✍️ Author

*   **Your Name**
    *   LinkedIn: [Your LinkedIn Profile](https://linkedin.com/in/yourprofile)
    *   Twitter: [@YourTwitter](https://twitter.com/YourTwitter)
    *   Website: [Your Personal Website](https://yourwebsite.com)

## 🏆 Badges

[![Rust Stable](https://img.shields.io/badge/rust-stable-blue.svg)](https://www.rust-lang.org/)
[![Crates.io](https://img.shields.io/crates/v/shared-types?label=shared--types)](https://crates.io/crates/shared-types)
[![Crates.io](https://img.shields.io/crates/v/shared-messaging?label=shared--messaging)](https://crates.io/crates/shared-messaging)
[![Crates.io](https://img.shields.io/crates/v/shared-utils?label=shared--utils)](https://crates.io/crates/shared-utils)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)