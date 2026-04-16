-- Migration 008: Add Soft Deletes
-- Appends `deleted_at TIMESTAMPTZ` to all critical resource tables.

ALTER TABLE organizations
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Rebuild organization status rule if needed, or leave it as it matches standard soft delete pattern.
-- The application logic will append `WHERE deleted_at IS NULL` to queries.

ALTER TABLE users
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE assets
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE accounts
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE invoices
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE listings
    ADD COLUMN deleted_at TIMESTAMPTZ;

ALTER TABLE clients
    ADD COLUMN deleted_at TIMESTAMPTZ;

-- Add indexes on deleted_at columns for query optimization since all `SELECT` queries will filter on it.
CREATE INDEX idx_organizations_deleted_at ON organizations (deleted_at);
CREATE INDEX idx_users_deleted_at         ON users (deleted_at);
CREATE INDEX idx_assets_deleted_at        ON assets (deleted_at);
CREATE INDEX idx_accounts_deleted_at      ON accounts (deleted_at);
CREATE INDEX idx_invoices_deleted_at      ON invoices (deleted_at);
CREATE INDEX idx_listings_deleted_at      ON listings (deleted_at);
CREATE INDEX idx_clients_deleted_at       ON clients (deleted_at);
