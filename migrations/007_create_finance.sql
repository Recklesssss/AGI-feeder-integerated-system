-- Migration 007: Finance — Accounts (double-entry ledger)
-- Chart of accounts per organization.

CREATE TABLE accounts (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL,
    account_type    TEXT        NOT NULL
                                CHECK (account_type IN ('asset', 'liability', 'equity', 'revenue', 'expense')),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_accounts_organization ON accounts (organization_id);
CREATE INDEX idx_accounts_type         ON accounts (account_type);

-- Migration 008: Finance — Invoices

CREATE TABLE invoices (
    id              UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID           NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    asset_id        UUID           REFERENCES assets (id) ON DELETE SET NULL,
    total           NUMERIC(15, 2) NOT NULL CHECK (total > 0),
    status          TEXT           NOT NULL DEFAULT 'draft'
                                   CHECK (status IN ('draft', 'issued', 'paid', 'cancelled')),
    issued_at       DATE,
    created_at      TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_invoices_organization ON invoices (organization_id);
CREATE INDEX idx_invoices_status       ON invoices (status);

-- Migration 009: Finance — Payments

CREATE TABLE payments (
    id         UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    invoice_id UUID           NOT NULL REFERENCES invoices (id) ON DELETE RESTRICT,
    amount     NUMERIC(15, 2) NOT NULL CHECK (amount > 0),
    method     TEXT           NOT NULL CHECK (method IN ('cash', 'bank_transfer', 'card', 'mobile')),
    paid_at    DATE,
    created_at TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_payments_invoice ON payments (invoice_id);

-- Migration 010: Finance — Ledger Entries (double-entry)

CREATE TABLE ledger_entries (
    id             UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    organization_id UUID          NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    account_id     UUID           NOT NULL REFERENCES accounts (id) ON DELETE RESTRICT,
    amount         NUMERIC(15, 2) NOT NULL CHECK (amount > 0),
    direction      TEXT           NOT NULL CHECK (direction IN ('debit', 'credit')),
    reference_type TEXT,          -- e.g. 'invoice', 'payment'
    reference_id   UUID,          -- FK to invoice/payment (polymorphic)
    created_at     TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_ledger_organization ON ledger_entries (organization_id);
CREATE INDEX idx_ledger_account      ON ledger_entries (account_id);
CREATE INDEX idx_ledger_reference    ON ledger_entries (reference_type, reference_id);
