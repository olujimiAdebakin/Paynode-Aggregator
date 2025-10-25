# ⚙️ Paynode Developer Setup

## Prerequisites
- Rust 1.75+  
- Docker & Docker Compose  
- PostgreSQL  
- Redis  
- NATS  

## Local Run

```bash
git clone https://github.com/olujimiAdebakin/paynode.git
cd paynode
cargo build --release
cargo run

Environment Variables

| Variable        | Description           |
| --------------- | --------------------- |
| `REDIS_URL`     | Redis instance URL    |
| `NATS_URL`      | NATS message bus      |
| `DATABASE_URL`  | PostgreSQL connection |
| `CHAIN_RPC_URL` | Blockchain RPC        |
| `AI_ROUTER_URL` | AI model endpoint     |
