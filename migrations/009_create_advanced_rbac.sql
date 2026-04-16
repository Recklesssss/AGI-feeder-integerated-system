-- Migration 009: Advanced RBAC Schema
-- Replaces the flat user_permissions mapping with a scalable Role/Group structure.

CREATE TABLE permissions (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    key         TEXT        NOT NULL UNIQUE, -- e.g. "VIEW_FINANCE", "CREATE_USER"
    description TEXT,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE roles (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL, -- e.g. "Manager", "Accountant"
    description     TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ,
    UNIQUE (organization_id, name)
);

CREATE TABLE role_permissions (
    role_id       UUID        NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    permission_id UUID        NOT NULL REFERENCES permissions (id) ON DELETE CASCADE,
    granted_at    TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (role_id, permission_id)
);

CREATE TABLE user_roles (
    user_id    UUID        NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    role_id    UUID        NOT NULL REFERENCES roles (id) ON DELETE CASCADE,
    granted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (user_id, role_id)
);

-- We leave `user_permissions` from 003 around during the transition phase,
-- or we can drop it. For now, we keep it as a legacy table or drop it immediately.
-- The spec asks to replace it.
DROP TABLE user_permissions;

CREATE INDEX idx_roles_org  ON roles (organization_id);
CREATE INDEX idx_user_roles ON user_roles (user_id);
