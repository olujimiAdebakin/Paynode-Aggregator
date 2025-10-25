
-- Orders table
CREATE TYPE order_status AS ENUM ('PENDING', 'ACCEPTED', 'FULFILLED', 'REFUNDED', 'EXPIRED');
CREATE TYPE order_tier AS ENUM ('ALPHA', 'BETA', 'DELTA', 'OMEGA', 'TITAN');

CREATE TABLE IF NOT EXISTS orders (
    id SERIAL PRIMARY KEY,
    order_id BYTEA UNIQUE NOT NULL,
    user_address BYTEA NOT NULL,
    token BYTEA NOT NULL,
    amount TEXT NOT NULL,
    refund_address BYTEA NOT NULL,
    integrator BYTEA NOT NULL,
    status order_status NOT NULL DEFAULT 'PENDING',
    tier order_tier,
    currency VARCHAR(10),
    block_number BIGINT NOT NULL,
    tx_hash BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMPTZ,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Provider intents
CREATE TABLE IF NOT EXISTS provider_intents (
    id SERIAL PRIMARY KEY,
    provider BYTEA NOT NULL,
    currency VARCHAR(10) NOT NULL,
    available_amount TEXT NOT NULL,
    min_fee_bps INTEGER NOT NULL,
    max_fee_bps INTEGER NOT NULL,
    commitment_window BIGINT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, currency)
);

-- Proposals
CREATE TYPE proposal_status AS ENUM ('PENDING', 'ACCEPTED', 'REJECTED', 'TIMED_OUT', 'EXECUTED');

CREATE TABLE IF NOT EXISTS proposals (
    id SERIAL PRIMARY KEY,
    proposal_id BYTEA UNIQUE NOT NULL,
    order_id BYTEA NOT NULL REFERENCES orders(order_id),
    provider BYTEA NOT NULL,
    proposed_fee_bps INTEGER NOT NULL,
    status proposal_status NOT NULL DEFAULT 'PENDING',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deadline TIMESTAMPTZ NOT NULL,
    accepted_at TIMESTAMPTZ,
    executed_at TIMESTAMPTZ,
    tx_hash BYTEA
);

-- Provider reputation
CREATE TABLE IF NOT EXISTS provider_reputation (
    provider BYTEA PRIMARY KEY,
    total_orders BIGINT NOT NULL DEFAULT 0,
    successful_orders BIGINT NOT NULL DEFAULT 0,
    failed_orders BIGINT NOT NULL DEFAULT 0,
    no_shows BIGINT NOT NULL DEFAULT 0,
    avg_settlement_time_seconds BIGINT NOT NULL DEFAULT 0,
    total_volume TEXT NOT NULL DEFAULT '0',
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexer checkpoint
CREATE TABLE IF NOT EXISTS indexer_checkpoint (
    id INTEGER PRIMARY KEY DEFAULT 1,
    last_block BIGINT NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT single_row CHECK (id = 1)
);

-- Indexes for performance
CREATE INDEX idx_orders_status ON orders(status);
CREATE INDEX idx_orders_user ON orders(user_address);
CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
CREATE INDEX idx_provider_intents_currency ON provider_intents(currency);
CREATE INDEX idx_provider_intents_active ON provider_intents(is_active, expires_at);
CREATE INDEX idx_proposals_order_id ON proposals(order_id);
CREATE INDEX idx_proposals_provider ON proposals(provider);
CREATE INDEX idx_proposals_status ON proposals(status);