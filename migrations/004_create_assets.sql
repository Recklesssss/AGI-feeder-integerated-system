-- Migration 004: Assets
-- Physical assets managed by the platform.

CREATE TABLE assets (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL,
    asset_type      TEXT        NOT NULL CHECK (asset_type IN ('PROPERTY', 'UNIT', 'RESTAURANT', 'LISTING')),
    status          TEXT        NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'INACTIVE', 'ARCHIVED')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_assets_organization ON assets (organization_id);
CREATE INDEX idx_assets_status       ON assets (status);
CREATE INDEX idx_assets_type         ON assets (asset_type);
