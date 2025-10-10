
---

### 🔌 `/docs/provider-spec.md`

> **Purpose:** Define how PSPs, OTC desks, or liquidity providers integrate into Paynode.

```markdown
# 🔌 Paynode Provider Integration Spec

## Overview

Providers are external participants (banks, PSPs, OTC desks) that receive and fulfill user orders via Paynode.

## Provider API Requirements

| Endpoint | Method | Description |
|-----------|---------|--------------|
| `/orders/assign` | POST | Receive a new order |
| `/orders/accept` | POST | Confirm you can process it |
| `/orders/fulfill` | POST | Report success and settlement TX |
| `/orders/fail` | POST | Report failure (auto-fallback triggered) |

## Example Payload

```json
{
  "orderId": "abc-123",
  "amount": 150000,
  "currency": "NGN",
  "recipient": {
    "bank": "GTBank",
    "account": "0123456789",
    "name": "John Doe"
  }
}


Provider Health

Providers periodically publish health pings:

{ "providerId": "A", "status": "online", "balance": 120000, "latency_ms": 90 }