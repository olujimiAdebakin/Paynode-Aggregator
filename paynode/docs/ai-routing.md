
---

### 🧠 `/docs/ai-routing.md`

> **Purpose:** Explain Paynode’s AI logic, provider scoring model, and how routing decisions are made in <100ms.

```markdown
# 🧠 Paynode AI Routing

The **AI Router** is Paynode’s decision engine — it selects the optimal provider for every order based on real-time data.

## Key Concepts

| Feature | Description |
|----------|--------------|
| **Feature Set** | 30+ metrics (success rate, latency, cost, uptime, load, geo-distance) |
| **Model** | LLM-assisted scoring model (Gemini Flash or equivalent) |
| **Latency** | <100ms decision |
| **Learning Loop** | Each transaction feeds back into the feature store |

## Routing Process

1. Fetch eligible providers (currency, balance, region).
2. Extract runtime metrics from Redis.
3. Send features to AI scoring endpoint.
4. Rank providers by probability of success.
5. Publish `order.assigned` event.

### Example Prompt to Model

```json
{
  "task": "score_providers",
  "currency": "NGN",
  "amount": 100,
  "region": "Nigeria",
  "features": [
    { "provider": "A", "success_24h": 0.96, "latency_ms": 85, "cost": 0.4 },
    { "provider": "B", "success_24h": 0.88, "latency_ms": 140, "cost": 0.2 }
  ]
}


Output → Provider A: 94.2 | Provider B: 78.5 → A assigned.

Learning

The AI Feedback Loop updates:

Success rate

Settlement latency

Load factor

Geographic reliability

These features directly affect future routing scores.