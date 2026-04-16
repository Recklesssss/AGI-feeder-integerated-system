-- Migration 002: Users
-- Platform users, scoped to an organization. Passwords are Argon2 hashed.

CREATE TABLE users (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    email           TEXT        NOT NULL UNIQUE,
    full_name       TEXT        NOT NULL,
    password_hash   TEXT        NOT NULL,
    status          TEXT        NOT NULL DEFAULT 'active'
                                CHECK (status IN ('active', 'inactive', 'locked', 'suspended')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_organization  ON users (organization_id);
CREATE INDEX idx_users_email         ON users (email);
CREATE INDEX idx_users_status        ON users (status);
