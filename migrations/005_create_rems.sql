-- Migration 005: REMS — Clients
-- Real estate clients (buyers, sellers, tenants, landlords).

CREATE TABLE clients (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL,   -- single name field (matches Client model)
    email           TEXT,
    phone           TEXT,
    client_type     TEXT        NOT NULL CHECK (client_type IN ('buyer', 'seller', 'lessee', 'lessor')),
    source          TEXT,
    status          TEXT        NOT NULL DEFAULT 'active' CHECK (status IN ('active', 'inactive', 'blacklisted')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_clients_organization ON clients (organization_id);
CREATE INDEX idx_clients_status       ON clients (status);

-- Migration 006: REMS — Listings
-- Property listings. No client_id (client is tracked at deal level, see T-10).

CREATE TABLE listings (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID           NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    asset_id        UUID           NOT NULL REFERENCES assets (id) ON DELETE RESTRICT,
    title           TEXT           NOT NULL,
    description     TEXT,
    price           NUMERIC(15, 2) NOT NULL CHECK (price >= 0),
    listing_type    TEXT           NOT NULL CHECK (listing_type IN ('sale', 'lease')),
    status          TEXT           NOT NULL DEFAULT 'draft'
                                   CHECK (status IN ('draft', 'active', 'sold', 'cancelled')),
    listed_at       DATE,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_listings_organization ON listings (organization_id);
CREATE INDEX idx_listings_asset        ON listings (asset_id);
CREATE INDEX idx_listings_status       ON listings (status);
