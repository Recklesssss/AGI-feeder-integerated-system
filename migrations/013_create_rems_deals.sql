-- Migration 013: REMS Deals & Commissions
-- Brokerage pipeline: deals link clients to listings, commissions track agent payouts.

CREATE TABLE deals (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID           NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    listing_id      UUID           NOT NULL REFERENCES listings (id) ON DELETE CASCADE,
    client_id       UUID           NOT NULL REFERENCES clients (id) ON DELETE CASCADE,
    deal_value      NUMERIC(15,2)  NOT NULL CHECK (deal_value > 0),
    status          TEXT           NOT NULL DEFAULT 'pending'
                                   CHECK (status IN ('pending', 'negotiation', 'closed_won', 'closed_lost')),
    closed_at       DATE,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE commissions (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    deal_id         UUID           NOT NULL REFERENCES deals (id) ON DELETE CASCADE,
    agent_id        UUID           NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    rate            NUMERIC(5,4)   NOT NULL CHECK (rate > 0 AND rate <= 1), -- e.g. 0.03 = 3%
    amount          NUMERIC(15,2)  NOT NULL CHECK (amount >= 0),
    paid            BOOLEAN        NOT NULL DEFAULT FALSE,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_deals_org        ON deals (organization_id);
CREATE INDEX idx_deals_listing    ON deals (listing_id);
CREATE INDEX idx_deals_client     ON deals (client_id);
CREATE INDEX idx_commissions_deal ON commissions (deal_id);
CREATE INDEX idx_commissions_agent ON commissions (agent_id);
