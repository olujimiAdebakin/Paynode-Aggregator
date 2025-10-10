# **Payflow - AI-Driven Payment Aggregator** 🚀

## Overview
PayNode is a robust, AI-driven payment aggregation platform engineered to streamline and optimize payment processing. This distributed system leverages a microservices architecture built with Rust, Axum, and Tokio, integrating with PostgreSQL for persistent data, Redis for caching, and NATS for inter-service communication to ensure high performance, scalability, and resilience.

## Features
-   **Microservices Architecture**: Decoupled services for enhanced scalability, maintainability, and fault isolation.
-   **AI-Driven Routing**: Intelligent routing of payment transactions to optimize provider selection and performance.
-   **Comprehensive Payment Processing**: Handles order creation, balance management, and settlement.
-   **Blockchain Integration**: Includes an indexer for monitoring and interacting with blockchain networks.
-   **Real-time Analytics**: Gathers and processes transactional data for actionable insights.
-   **Asynchronous Communication**: Utilizes NATS for efficient, event-driven communication between services.
-   **Containerization**: Docker Compose setup for easy local development and deployment.

## Getting Started

### Installation
Follow these steps to get Payflow up and running on your local machine.

-   **Clone the Repository**:
    ```bash
    git clone https://github.com/your-username/paynode.git # Replace with your actual repository URL
    cd paynode
    ```

-   **Build Project Dependencies**:
    Navigate to the `paynode` directory and build the Rust workspace.
    ```bash
    cd paynode
    cargo build
    ```

-   **Start Infrastructure Services**:
    Use Docker Compose to launch PostgreSQL, Redis, and NATS.
    ```bash
    docker-compose up -d
    ```

-   **Configure Environment Variables**:
    Copy the example environment file and populate it with your specific configurations.
    ```bash
    cp .env.example .env
    ```
    Edit the newly created `.env` file to set actual values for `GEMINI_API_KEY` and other service ports if needed.

### Environment Variables
The following environment variables are required for the application to function correctly. Please ensure your `.env` file contains these values.

-   `DATABASE_URL`: PostgreSQL connection string.
    _Example_: `postgresql://postgres:postgres@localhost:5432/payflow`
-   `REDIS_URL`: Redis connection string.
    _Example_: `redis://localhost:6379`
-   `NATS_URL`: NATS server connection string.
    _Example_: `nats://localhost:4222`
-   `GEMINI_API_KEY`: API key for the AI routing service.
    _Example_: `your_api_key_here`
-   `API_GATEWAY_PORT`: Port for the API Gateway service.
    _Example_: `8000`
-   `ORDER_SERVICE_PORT`: Port for the Order Service.
    _Example_: `8001`
-   `AI_ROUTER_PORT`: Port for the AI Router Service.
    _Example_: `8002`
-   `SHARD_ID`: Identifier for sharding (if applicable in future distributed setups).
    _Example_: `1`
-   `SUPPORTED_CURRENCIES`: Comma-separated list of supported currencies.
    _Example_: `NGN,GHS,KES`

## Usage

To start individual services, navigate to the `paynode` directory and use `cargo run` with the appropriate binary name.

-   **Run API Gateway**:
    ```bash
    cargo run --bin api-gateway
    ```

-   **Run Order Service**:
    ```bash
    cargo run --bin order-service
    ```

-   **Run AI Router Service**:
    ```bash
    cargo run --bin ai-router
    ```

-   **Run Provider Service**:
    ```bash
    cargo run --bin provider-service
    ```

-   **Run Balance Service**:
    ```bash
    cargo run --bin balance-service
    ```

-   **Run Settlement Service**:
    ```bash
    cargo run --bin settlement-service
    ```

-   **Run Analytics Service**:
    ```bash
    cargo run --bin analytics-service
    ```

-   **Run Blockchain Indexer Service**:
    ```bash
    cargo run --bin blockchain-indexer
    ```

### Running Tests
To execute the project's test suite:
```bash
cargo test
```

## Technologies Used

| Technology         | Description                                     | Link                                                            |
| :----------------- | :---------------------------------------------- | :-------------------------------------------------------------- |
| **Rust**           | Primary programming language for performance    | [rust-lang.org](https://www.rust-lang.org/)                     |
| **Axum**           | Web framework for API Gateway                   | [docs.rs/axum](https://docs.rs/axum)                            |
| **Tokio**          | Asynchronous runtime for concurrent operations  | [tokio.rs](https://tokio.rs/)                                   |
| **SQLx**           | Asynchronous SQL toolkit for PostgreSQL         | [github.com/launchbadge/sqlx](https://github.com/launchbadge/sqlx) |
| **PostgreSQL**     | Relational database for core data storage       | [postgresql.org](https://www.postgresql.org/)                   |
| **Redis**          | In-memory data store for caching & state        | [redis.io](https://redis.io/)                                   |
| **NATS**           | High-performance messaging system               | [nats.io](https://nats.io/)                                     |
| **Serde**          | Serialization/deserialization framework         | [serde.rs](https://serde.rs/)                                   |
| **uuid**           | Universally Unique Identifier generation        | [docs.rs/uuid](https://docs.rs/uuid)                            |
| **chrono**         | Date and time functionalities                   | [docs.rs/chrono](https://docs.rs/chrono)                        |
| **rust_decimal**   | Arbitrary-precision decimal arithmetic          | [docs.rs/rust_decimal](https://docs.rs/rust_decimal)            |
| **tracing**        | Structured logging and diagnostics              | [docs.rs/tracing](https://docs.rs/tracing)                      |
| **anyhow**         | Flexible error handling                         | [docs.rs/anyhow](https://docs.rs/anyhow)                        |
| **dotenv**         | Loading environment variables from `.env` file  | [docs.rs/dotenv](https://docs.rs/dotenv)                        |
| **reqwest**        | Asynchronous HTTP client                        | [docs.rs/reqwest](https://docs.rs/reqwest)                      |
| **ethers-rs**      | Ethereum client libraries for blockchain tasks  | [docs.rs/ethers](https://docs.rs/ethers)                        |
| **Docker Compose** | Container orchestration for local development   | [docs.docker.com](https://docs.docker.com/compose/)             |

# Payflow API

## Overview
The PayNode API Gateway, built with Rust and the Axum framework, acts as the entry point for all client requests into the PayNode microservices ecosystem. It provides essential routing and health monitoring functionalities, ensuring system availability.

## Features
-   **Axum**: Handles HTTP routing and request/response processing.
-   **Tokio**: Provides an asynchronous runtime for high-performance I/O operations.
-   **Tower-HTTP**: Integrates middleware for CORS and tracing.
-   **Dotenv**: Manages environment variables for configuration.

## Getting Started
### Installation
The API Gateway is part of the larger Payflow project. Ensure you have followed the main project's installation steps outlined above. Specifically, after cloning the repository and building dependencies, navigate to the `paynode` directory.

### Environment Variables
-   `API_GATEWAY_PORT`: The port on which the API Gateway will listen for incoming requests.
    _Example_: `8000`

## API Documentation
### Base URL
The base URL for the API Gateway is typically `http://localhost:[API_GATEWAY_PORT]`, which defaults to `http://localhost:8000` during local development.

### Endpoints
#### GET /health
**Description**: Checks the operational status of the API Gateway.
**Request**:
No payload required.

**Response**:
```text
OK
```

**Errors**:
-   `500 Internal Server Error`: The service is not running or encountered an unexpected issue during startup.

## Contributing
We welcome contributions to the Payflow project! To get started:

-   🌿 Fork the repository and create a new branch for your feature or bug fix.
-   🛠️ Ensure your code adheres to existing coding standards and passes all tests.
-   📝 Write clear, concise commit messages.
-   ✅ Open a pull request with a detailed description of your changes.

## License
This project is currently unlicensed. Please consult the repository owner for licensing information.

## Author Info
-   Your Name: [LinkedIn](https://linkedin.com/in/your-profile) | [Twitter](https://twitter.com/your-twitter)

## Badges
[![Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org/)
[![Web Framework](https://img.shields.io/badge/Web%20Framework-Axum-blue)](https://docs.rs/axum/latest/axum/)
[![Database](https://img.shields.io/badge/Database-PostgreSQL-blue.svg)](https://www.postgresql.org/)
[![Messaging](https://img.shields.io/badge/Messaging-NATS-blueviolet)](https://nats.io/)
[![Containerization](https://img.shields.io/badge/Container-Docker-informational)](https://www.docker.com/)

[![Readme was generated by Dokugen](https://img.shields.io/badge/Readme%20was%20generated%20by-Dokugen-brightgreen)](https://www.npmjs.com/package/dokugen)