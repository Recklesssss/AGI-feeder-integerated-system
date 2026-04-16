-- Migration 010: Audit Trail
-- Immutable log of all data mutations across every bounded context.
-- This table is strictly INSERT-only. No UPDATE or DELETE operations are allowed.

CREATE TABLE audit_logs (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    user_id         UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    action          TEXT        NOT NULL CHECK (action IN ('create', 'update', 'delete')),
    entity          TEXT        NOT NULL, -- e.g. 'asset', 'invoice', 'user', 'listing'
    entity_id       UUID        NOT NULL,
    metadata        JSONB       NOT NULL DEFAULT '{}',
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_audit_org        ON audit_logs (organization_id);
CREATE INDEX idx_audit_user       ON audit_logs (user_id);
CREATE INDEX idx_audit_entity     ON audit_logs (entity, entity_id);
CREATE INDEX idx_audit_created_at ON audit_logs (created_at);
