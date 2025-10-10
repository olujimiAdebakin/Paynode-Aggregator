---

## 🧩 System Architecture Breakdown

Paynode is composed of **modular Rust microservices** connected by an async message bus (NATS).  
Each service performs a single responsibility and communicates via events, ensuring **horizontal scalability, fault isolation, and zero shared state**.

---

### **1. API Gateway**

**Role:** Entry point for all client requests (dApps, PSPs, SDKs).  
**Responsibilities:**
- Validates incoming orders and user signatures.  
- Forwards valid requests to the `Order Service`.  
- Handles authentication, rate limiting, and versioning.  

**Interfaces:**
- REST / GraphQL for clients.  
- Publishes `order.created` event to NATS.  

**Tech:** Actix Web + JWT Auth + OpenAPI spec.

---

### **2. Smart Contract Escrow Layer**

**Role:** On-chain settlement and fund safety mechanism.  
**Responsibilities:**
- Locks funds upon order creation.  
- Releases to the provider after fulfillment confirmation.  
- Emits blockchain events (captured by Indexer).  

**Contracts:**
- `OrderEscrow.sol` (Base / Polygon)  
- Events: `OrderCreated`, `OrderSettled`, `OrderFailed`

**Guarantee:** Paynode never holds user funds — non-custodial by design.

---

### **3. Blockchain Indexer**

**Role:** Bridges on-chain events to off-chain services.  
**Responsibilities:**
- Listens for events from smart contracts.  
- Verifies confirmations and publishes them to NATS.  

**Events Published:**
- `order.created` → triggers order creation flow.  
- `order.settled` → triggers settlement reconciliation.  

**Tech:** Rust + ethers-rs + WebSocket streaming.

---

### **4. Order Service**

**Role:** Manages lifecycle of payment orders.  
**Responsibilities:**
- Creates order records (Postgres).  
- Tracks status: `Pending → Processing → Validated → Settled`.  
- Emits events:  
  - `order.pending`  
  - `order.assigned`  
  - `order.validated`

**Storage:** PostgreSQL + Redis for caching.

---

### **5. AI Router Service**

**Role:** Core intelligence layer — decides *who gets the order*.  
**Responsibilities:**
- Fetches eligible providers for a currency pair.  
- Extracts 30+ features (success rate, latency, cost, distance, uptime).  
- Scores providers using LLM or model endpoint (e.g., Gemini Flash).  
- Publishes best match to `order.assigned`.

**Routing Logic:**
```rust
if let Some(best) = providers.iter()
    .map(|p| (p.id, ai.score(p)))
    .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()) {
        assign_order(best.0);
}

Performance: <100ms decision latency.

6. Provider Service

Role: Interface to connected PSPs, OTC desks, and local agents.
Responsibilities:

Reserves provider liquidity.

Initiates payment execution (bank API, mobile money, etc.).

Reports results back to Order Service.

Integration Examples:

Paystack, Flutterwave, Opay, M-Pesa, Circle, Binance Connect.

Events:

order.fulfilled (success)

order.failed (retry / fallback)

7. Settlement Service

Role: Completes final fund movement and reconciles transaction states.
Responsibilities:

Reads fulfilled orders.

Executes blockchain settlement transaction via escrow contract.

Monitors confirmations and updates DB.

Optimization:

Chooses cheapest gas route automatically.

Batches settlements when possible.


Metrics Tracked:

| Metric               | Window    | Use                |
| -------------------- | --------- | ------------------ |
| Success Rate         | 24h / 7d  | Routing weight     |
| Latency              | p95       | Scoring feature    |
| Provider Load        | Real-time | Balancing          |
| Regional Reliability | 7d        | Shard optimization |


Event Flow Summary

| Step | Event             | Producer           | Consumer           |
| ---- | ----------------- | ------------------ | ------------------ |
| 1    | `OrderCreated`    | Smart Contract     | Indexer            |
| 2    | `order.created`   | Indexer            | Order Service      |
| 3    | `order.pending`   | Order Service      | AI Router          |
| 4    | `order.assigned`  | AI Router          | Provider Service   |
| 5    | `order.fulfilled` | Provider Service   | Settlement Service |
| 6    | `order.settled`   | Settlement Service | AI Feedback Loop   |



🧠 Scaling Principles

Stateless Services: Each service is independently deployable.

Message Bus Communication: All async → no blocking RPCs.

Shard by Currency: Each region/currency runs isolated — shard_ngn, shard_usd, etc.

Predictive Load Balancing: AI uses provider health + corridor metrics to shift load.

Kubernetes Orchestration: Auto-scales based on traffic and provider density.

🧩 Key Design Philosophies

AI over Static Logic → The router learns, it doesn’t guess.

Horizontal, not Vertical → Scale by adding nodes, not upgrading one.

Event-driven State → Everything is publish/subscribe.

Non-custodial by Default → Smart contracts hold funds, not servers.

Resilient-by-Design → Partial failures don’t collapse the network.



📊 Metrics & Observability
| Metric             | Target   | Notes                   |
| ------------------ | -------- | ----------------------- |
| Avg Routing Time   | <100ms   | AI inference latency    |
| Settlement Time    | <2 min   | End-to-end              |
| Order Success Rate | >95%     | Adaptive AI routing     |
| Uptime             | 99.99%   | Multi-region redundancy |
| Concurrent Users   | 100,000+ | Stress-tested shards    |



🔍 In Summary

Paynode isn’t a payment processor — it’s a payment intelligence fabric.
Each transaction teaches the network how to route better, faster, and cheaper.
Built for global scale, engineered in Rust, and designed for autonomy.