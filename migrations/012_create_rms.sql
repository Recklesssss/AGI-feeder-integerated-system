-- Migration 012: Restaurant Management System (RMS)
-- POS logic: restaurants, menus, orders, inventory, and stock movements.

CREATE TABLE restaurants (
    id              UUID        PRIMARY KEY DEFAULT gen_random_uuid(),
    asset_id        UUID        NOT NULL UNIQUE REFERENCES assets (id) ON DELETE CASCADE,
    organization_id UUID        NOT NULL REFERENCES organizations (id) ON DELETE CASCADE,
    name            TEXT        NOT NULL,
    address         TEXT,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    deleted_at      TIMESTAMPTZ
);

CREATE TABLE menu_items (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id UUID           NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
    name          TEXT           NOT NULL,
    description   TEXT,
    price         NUMERIC(15,2)  NOT NULL CHECK (price >= 0),
    category      TEXT,
    is_available  BOOLEAN        NOT NULL DEFAULT TRUE,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ
);

CREATE TABLE orders (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id UUID           NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
    table_number  TEXT,
    total         NUMERIC(15,2)  NOT NULL DEFAULT 0,
    status        TEXT           NOT NULL DEFAULT 'pending'
                                 CHECK (status IN ('pending', 'paid', 'cancelled')),
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE TABLE order_items (
    id           UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id     UUID           NOT NULL REFERENCES orders (id) ON DELETE CASCADE,
    menu_item_id UUID           NOT NULL REFERENCES menu_items (id) ON DELETE RESTRICT,
    quantity     INT            NOT NULL CHECK (quantity > 0),
    unit_price   NUMERIC(15,2)  NOT NULL,
    subtotal     NUMERIC(15,2)  NOT NULL
);

CREATE TABLE inventory_items (
    id            UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    restaurant_id UUID           NOT NULL REFERENCES restaurants (id) ON DELETE CASCADE,
    name          TEXT           NOT NULL,
    unit          TEXT           NOT NULL DEFAULT 'kg',  -- kg, liters, units
    quantity      NUMERIC(15,3)  NOT NULL DEFAULT 0,
    reorder_level NUMERIC(15,3)  NOT NULL DEFAULT 0,
    created_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    updated_at    TIMESTAMPTZ    NOT NULL DEFAULT NOW(),
    deleted_at    TIMESTAMPTZ
);

CREATE TABLE stock_movements (
    id                UUID           PRIMARY KEY DEFAULT gen_random_uuid(),
    inventory_item_id UUID           NOT NULL REFERENCES inventory_items (id) ON DELETE CASCADE,
    movement_type     TEXT           NOT NULL CHECK (movement_type IN ('in', 'out', 'adjustment')),
    quantity          NUMERIC(15,3)  NOT NULL,
    reason            TEXT,
    created_at        TIMESTAMPTZ    NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_restaurants_org       ON restaurants (organization_id);
CREATE INDEX idx_menu_items_restaurant ON menu_items (restaurant_id);
CREATE INDEX idx_orders_restaurant     ON orders (restaurant_id);
CREATE INDEX idx_order_items_order     ON order_items (order_id);
CREATE INDEX idx_inventory_restaurant  ON inventory_items (restaurant_id);
CREATE INDEX idx_stock_movements_item  ON stock_movements (inventory_item_id);
