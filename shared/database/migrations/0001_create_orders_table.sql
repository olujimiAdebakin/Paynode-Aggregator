-- Add migration script here

-- -- Orders table
-- CREATE TYPE order_status AS ENUM ('PENDING', 'ACCEPTED', 'FULFILLED', 'REFUNDED', 'EXPIRED');
-- CREATE TYPE order_tier AS ENUM ('ALPHA', 'BETA', 'DELTA', 'OMEGA', 'TITAN');

-- CREATE TABLE IF NOT EXISTS orders (
--     id SERIAL PRIMARY KEY,
--     order_id BYTEA UNIQUE NOT NULL,
--     user_address BYTEA NOT NULL,
--     token BYTEA NOT NULL,
--     amount TEXT NOT NULL,
--     refund_address BYTEA NOT NULL,
--     integrator BYTEA NOT NULL,
--     status order_status NOT NULL DEFAULT 'PENDING',
--     tier order_tier,
--     currency VARCHAR(10),
--     block_number BIGINT NOT NULL,
--     tx_hash BYTEA NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     expires_at TIMESTAMPTZ,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );

-- -- Provider intents
-- CREATE TABLE IF NOT EXISTS provider_intents (
--     id SERIAL PRIMARY KEY,
--     provider BYTEA NOT NULL,
--     currency VARCHAR(10) NOT NULL,
--     available_amount TEXT NOT NULL,
--     min_fee_bps INTEGER NOT NULL,
--     max_fee_bps INTEGER NOT NULL,
--     commitment_window BIGINT NOT NULL,
--     is_active BOOLEAN NOT NULL DEFAULT true,
--     expires_at TIMESTAMPTZ NOT NULL,
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     UNIQUE(provider, currency)
-- );

-- -- Proposals
-- CREATE TYPE proposal_status AS ENUM ('PENDING', 'ACCEPTED', 'REJECTED', 'TIMED_OUT', 'EXECUTED');

-- CREATE TABLE IF NOT EXISTS proposals (
--     id SERIAL PRIMARY KEY,
--     proposal_id BYTEA UNIQUE NOT NULL,
--     order_id BYTEA NOT NULL REFERENCES orders(order_id),
--     provider BYTEA NOT NULL,
--     proposed_fee_bps INTEGER NOT NULL,
--     status proposal_status NOT NULL DEFAULT 'PENDING',
--     created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     deadline TIMESTAMPTZ NOT NULL,
--     accepted_at TIMESTAMPTZ,
--     executed_at TIMESTAMPTZ,
--     tx_hash BYTEA
-- );

-- -- Provider reputation
-- CREATE TABLE IF NOT EXISTS provider_reputation (
--     provider BYTEA PRIMARY KEY,
--     total_orders BIGINT NOT NULL DEFAULT 0,
--     successful_orders BIGINT NOT NULL DEFAULT 0,
--     failed_orders BIGINT NOT NULL DEFAULT 0,
--     no_shows BIGINT NOT NULL DEFAULT 0,
--     avg_settlement_time_seconds BIGINT NOT NULL DEFAULT 0,
--     total_volume TEXT NOT NULL DEFAULT '0',
--     last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW()
-- );

-- -- Indexer checkpoint
-- CREATE TABLE IF NOT EXISTS indexer_checkpoint (
--     id INTEGER PRIMARY KEY DEFAULT 1,
--     last_block BIGINT NOT NULL,
--     updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
--     CONSTRAINT single_row CHECK (id = 1)
-- );

-- -- Indexes for performance
-- CREATE INDEX idx_orders_status ON orders(status);
-- CREATE INDEX idx_orders_user ON orders(user_address);
-- CREATE INDEX idx_orders_created_at ON orders(created_at DESC);
-- CREATE INDEX idx_provider_intents_currency ON provider_intents(currency);
-- CREATE INDEX idx_provider_intents_active ON provider_intents(is_active, expires_at);
-- CREATE INDEX idx_proposals_order_id ON proposals(order_id);
-- CREATE INDEX idx_proposals_provider ON proposals(provider);
-- CREATE INDEX idx_proposals_status ON proposals(status);



-- ------------------------------------------------------------
-- 1. ENUM types
-- ------------------------------------------------------------
CREATE TYPE order_status AS ENUM ('PENDING', 'ACCEPTED', 'FULFILLED', 'REFUNDED', 'EXPIRED');
CREATE TYPE order_tier   AS ENUM ('ALPHA', 'BETA', 'DELTA', 'OMEGA', 'TITAN');
CREATE TYPE proposal_status AS ENUM ('PENDING', 'ACCEPTED', 'REJECTED', 'TIMED_OUT', 'EXECUTED');

-- ------------------------------------------------------------
-- 2. Tables
-- ------------------------------------------------------------


CREATE TABLE IF NOT EXISTS integrator_fees (
    integrator_address BYTEA       PRIMARY KEY,
    fee_bps            INTEGER     NOT NULL DEFAULT 50,  -- Basis points (0.50% default)
    created_at         TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);



CREATE TABLE IF NOT EXISTS orders (
    id               SERIAL       PRIMARY KEY,
    order_id         BYTEA        UNIQUE NOT NULL,
    user_address     BYTEA        NOT NULL,
    token            BYTEA        NOT NULL,
    amount           TEXT         NOT NULL,
    refund_address   BYTEA        NOT NULL,
    integrator_address       BYTEA        NOT NULL,
    integrator_fees    INTEGER  NOT NULL,
    status           order_status NOT NULL DEFAULT 'PENDING',
    tier             order_tier,
    currency         VARCHAR(10),
    block_number     BIGINT       NOT NULL,
    tx_hash          BYTEA        NOT NULL,
    created_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW(),
    expires_at       TIMESTAMPTZ,
    updated_at       TIMESTAMPTZ  NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS provider_intents (
    id                  SERIAL      PRIMARY KEY,
    provider            BYTEA       NOT NULL,
    currency            VARCHAR(10) NOT NULL,
    available_amount    TEXT        NOT NULL,
    min_fee_bps         INTEGER     NOT NULL,
    max_fee_bps         INTEGER     NOT NULL,
    commitment_window   BIGINT      NOT NULL,
    is_active           BOOLEAN     NOT NULL DEFAULT true,
    expires_at          TIMESTAMPTZ NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(provider, currency)
);

CREATE TABLE IF NOT EXISTS proposals (
    id               SERIAL        PRIMARY KEY,
    proposal_id      BYTEA         UNIQUE NOT NULL,
    order_id         BYTEA         NOT NULL REFERENCES orders(order_id) ON DELETE CASCADE,
    provider         BYTEA         NOT NULL,
    proposed_fee_bps INTEGER       NOT NULL,
    status           proposal_status NOT NULL DEFAULT 'PENDING',
    created_at       TIMESTAMPTZ   NOT NULL DEFAULT NOW(),
    deadline         TIMESTAMPTZ   NOT NULL,
    accepted_at      TIMESTAMPTZ,
    executed_at      TIMESTAMPTZ,
    tx_hash          BYTEA
);

CREATE TABLE IF NOT EXISTS provider_reputation (
    provider                     BYTEA       PRIMARY KEY,
    total_orders                 BIGINT      NOT NULL DEFAULT 0,
    successful_orders            BIGINT      NOT NULL DEFAULT 0,
    failed_orders                BIGINT      NOT NULL DEFAULT 0,
    no_shows                     BIGINT      NOT NULL DEFAULT 0,
    avg_settlement_time_seconds  BIGINT      NOT NULL DEFAULT 0,
    total_volume                 TEXT        NOT NULL DEFAULT '0',
    last_updated                 TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS indexer_checkpoint (
    id          INTEGER     PRIMARY KEY DEFAULT 1,
    last_block  BIGINT      NOT NULL,
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT single_row CHECK (id = 1)
);

-- ------------------------------------------------------------
-- 3. Indexes (exactly the ones you listed)
-- ------------------------------------------------------------
CREATE INDEX IF NOT EXISTS idx_orders_status          ON orders(status);

CREATE INDEX IF NOT EXISTS idx_orders_user            ON orders(user_address);
CREATE INDEX IF NOT EXISTS idx_orders_created_at      ON orders(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_provider_intents_currency ON provider_intents(currency);
CREATE INDEX IF NOT EXISTS idx_provider_intents_active   ON provider_intents(is_active, expires_at);
CREATE INDEX IF NOT EXISTS idx_proposals_order_id     ON proposals(order_id);
CREATE INDEX IF NOT EXISTS idx_proposals_provider     ON proposals(provider);
CREATE INDEX IF NOT EXISTS idx_proposals_status       ON proposals(status);

CREATE INDEX IF NOT EXISTS idx_integrator_fees_address ON integrator_fees(integrator_address);
-- ------------------------------------------------------------
-- 4. updated_at trigger (used by your UPDATE statements)
-- ------------------------------------------------------------
CREATE OR REPLACE FUNCTION update_updated_at()
RETURNS TRIGGER AS $$
BEGIN
   NEW.updated_at = NOW();
   RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_integrator_fees_updated_at
    BEFORE UPDATE ON integrator_fees
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_orders_updated_at
    BEFORE UPDATE ON orders
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_provider_intents_updated_at
    BEFORE UPDATE ON provider_intents
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_proposals_updated_at
    BEFORE UPDATE ON proposals
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

CREATE TRIGGER trg_provider_reputation_updated_at
    BEFORE UPDATE ON provider_reputation
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();

-- (optional) indexer_checkpoint also gets a trigger if you ever UPDATE it
CREATE TRIGGER trg_indexer_checkpoint_updated_at
    BEFORE UPDATE ON indexer_checkpoint
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at();