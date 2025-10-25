
---

### 💸 `/docs/settlement.md`

> **Purpose:** Explain escrow logic, blockchain flow, and fund release mechanism.

```markdown
# 💸 Paynode Settlement Flow

Paynode uses **smart contract escrow** for non-custodial fund management.

## Process

1. User creates order → funds locked in contract.
2. AI assigns provider → order executed off-chain.
3. Provider reports fulfillment → contract releases funds.

## Smart Contract Events

| Event | Description |
|--------|-------------|
| `OrderCreated` | Funds escrowed |
| `OrderFulfilled` | Provider executed |
| `OrderSettled` | On-chain release complete |

## Multi-Chain Design

- **Primary:** Base Network (low gas)  
- **Fallback:** Polygon  
- **Future:** LayerZero / Wormhole bridge support

Each settlement transaction is signed by protocol wallet and verified by the settlement service.
