-- Migration 011: Property Management System (PMS)
-- Links properties to assets, then cascades into units, tenants, leases, and maintenance.

CREATE TABLE properties (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id        UUID        NOT NULL UNIQUE REFERENCES assets (id) ON DELETE CASCADE,
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    address         TEXT        NOT NULL,
    property_type   TEXT        NOT NULL CHECK (property_type IN ('residential', 'commercial', 'mixed')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE units (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    property_id UUID        NOT NULL REFERENCES properties (id) ON DELETE CASCADE,
    label       TEXT        NOT NULL,  -- e.g. "Unit 3B", "Suite 201"
    bedrooms    INT,
    bathrooms   INT,
    area_sqm    NUMERIC(10,2),
    rent_amount NUMERIC(15,2),
    status      TEXT        NOT NULL DEFAULT 'vacant'
                            CHECK (status IN ('vacant', 'occupied', 'maintenance')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);

CREATE TABLE tenants (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL,
    email           TEXT,
    phone           TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE leases (
    id          UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id     UUID           NOT NULL REFERENCES units (id) ON DELETE CASCADE,
    tenant_id   UUID           NOT NULL REFERENCES tenants (id) ON DELETE CASCADE,
    start_date  DATE           NOT NULL,
    end_date    DATE           NOT NULL,
    rent        NUMERIC(15,2)  NOT NULL CHECK (rent > 0),
    deposit     NUMERIC(15,2)  NOT NULL DEFAULT 0,
    status      TEXT           NOT NULL DEFAULT 'active'
                               CHECK (status IN ('active', 'terminated', 'expired')),
    created_at  TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);

CREATE TABLE maintenance_requests (
    id          UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    unit_id     UUID        NOT NULL REFERENCES units (id) ON DELETE CASCADE,
    reported_by UUID        REFERENCES tenants (id) ON DELETE SET NULL,
    title       TEXT        NOT NULL,
    description TEXT,
    priority    TEXT        NOT NULL DEFAULT 'medium'
                            CHECK (priority IN ('low', 'medium', 'high', 'urgent')),
    status      TEXT        NOT NULL DEFAULT 'open'
                            CHECK (status IN ('open', 'in_progress', 'resolved', 'closed')),
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at  TIMESTAMPTZ
);

CREATE INDEX idx_properties_org      ON properties (organization_id);
CREATE INDEX idx_units_property      ON units (property_id);
CREATE INDEX idx_tenants_org         ON tenants (organization_id);
CREATE INDEX idx_leases_unit         ON leases (unit_id);
CREATE INDEX idx_leases_tenant       ON leases (tenant_id);
CREATE INDEX idx_maintenance_unit    ON maintenance_requests (unit_id);
