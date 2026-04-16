-- Migration 003: RBAC — User Permissions
-- Fine-grained permission keys stored per user.
-- Keys must be uppercase strings matching the Permission enum in rbac::model.

CREATE TABLE user_permissions (
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    permission TEXT        NOT NULL,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, permission)
);

CREATE INDEX idx_user_permissions_user ON user_permissions (user_id);

-- Seed: valid permission keys are enforced at the application layer (rbac::mapper).
-- Examples: VIEW_USERS, CREATE_USER, VIEW_FINANCE, CREATE_PROPERTY, etc.
