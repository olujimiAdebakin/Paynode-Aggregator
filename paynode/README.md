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
