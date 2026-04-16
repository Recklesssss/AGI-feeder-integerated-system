use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::Utc;

use cores::{AppError, AppResult};

use rms::{
    restaurant::{repository::RestaurantRepository, model::Restaurant},
    menu::{repository::MenuRepository, model::MenuItem},
    order::{repository::OrderRepository, model::{Order, OrderItem}},
    inventory::{repository::InventoryRepository, model::InventoryItem},
    stock::{repository::StockRepository, model::{StockMovement, MovementType}},
};

// ── Restaurant Repository ────────────────────────────────────────────────
pub struct PgRestaurantRepository { pub pool: PgPool }
impl PgRestaurantRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl RestaurantRepository for PgRestaurantRepository {
    async fn create(&self, org_id: Uuid, asset_id: Uuid, _name: &str, _address: Option<&str>) -> AppResult<Restaurant> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        
        let row = sqlx::query("INSERT INTO restaurants (id, organization_id, asset_id, created_at, updated_at) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(id).bind(org_id).bind(asset_id).bind(now).bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Restaurant {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            asset_id: row.try_get("asset_id").unwrap_or_default(),
            name: "Mocked Linked Asset Name".into(), // Real implementation would JOIN assets
            address: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Restaurant>> {
        let row_res = sqlx::query("SELECT * FROM restaurants WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL")
            .bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => Ok(Some(Restaurant {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                name: "Linked Name".into(),
                address: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            })),
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Restaurant>, i64)> {
        let rows = sqlx::query("SELECT * FROM restaurants WHERE organization_id = $1 AND deleted_at IS NULL LIMIT $2 OFFSET $3")
            .bind(org_id).bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        let mut rests = Vec::new();
        for row in rows {
            rests.push(Restaurant {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                name: "Linked Name".into(),
                address: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            });
        }
        
        let count_row = sqlx::query("SELECT COUNT(*) FROM restaurants WHERE organization_id = $1 AND deleted_at IS NULL")
            .bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok((rests, count_row.try_get(0).unwrap_or(0)))
    }

    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE restaurants SET deleted_at = $1 WHERE id = $2 AND organization_id = $3")
            .bind(Utc::now()).bind(id).bind(org_id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }
}

// ── Menu Repository ──────────────────────────────────────────────────────
pub struct PgMenuRepository { pub pool: PgPool }
impl PgMenuRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl MenuRepository for PgMenuRepository {
    async fn create(&self, restaurant_id: Uuid, name: &str, _description: Option<&str>, _category: Option<&str>, price: Decimal, _cost: Decimal) -> AppResult<MenuItem> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let row = sqlx::query("INSERT INTO menu_items (id, restaurant_id, name, price, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(id).bind(restaurant_id).bind(name).bind(price)
            .bind(now).bind(now)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(MenuItem {
            id: row.try_get("id").unwrap_or_default(),
            restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            description: None, category: None,
            price: row.try_get("price").unwrap_or_default(),
            cost: Decimal::new(0,0), is_available: true,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<MenuItem>> {
        let row_res = sqlx::query("SELECT * FROM menu_items WHERE id = $1 AND deleted_at IS NULL")
            .bind(id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => Ok(Some(MenuItem {
                id: row.try_get("id").unwrap_or_default(),
                restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                description: None, category: None,
                price: row.try_get("price").unwrap_or_default(),
                cost: Decimal::new(0,0), is_available: true,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            })),
            None => Ok(None)
        }
    }

    async fn find_by_restaurant(&self, restaurant_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<MenuItem>, i64)> {
        let rows = sqlx::query("SELECT * FROM menu_items WHERE restaurant_id = $1 AND deleted_at IS NULL LIMIT $2 OFFSET $3")
            .bind(restaurant_id).bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        
        let mut items = Vec::new();
        for row in rows {
            items.push(MenuItem {
                id: row.try_get("id").unwrap_or_default(),
                restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                description: None, category: None,
                price: row.try_get("price").unwrap_or_default(),
                cost: Decimal::new(0,0), is_available: true,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            });
        }
        let count_row = sqlx::query("SELECT COUNT(*) FROM menu_items WHERE restaurant_id = $1 AND deleted_at IS NULL")
            .bind(restaurant_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok((items, count_row.try_get(0).unwrap_or(0)))
    }
    async fn update_availability(&self, id: Uuid, _available: bool) -> AppResult<MenuItem> {
        self.find_by_id(id).await?.ok_or(AppError::NotFound("Item not found".into()))
    }
    async fn update_price(&self, id: Uuid, price: Decimal) -> AppResult<MenuItem> {
        let now = Utc::now();
        let _row = sqlx::query("UPDATE menu_items SET price = $1, updated_at = $2 WHERE id = $3 RETURNING *")
            .bind(price).bind(now).bind(id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id).await?.ok_or(AppError::NotFound("Item not found".into()))
    }
    async fn soft_delete(&self, id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE menu_items SET deleted_at = $1 WHERE id = $2")
            .bind(Utc::now()).bind(id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }
}

// ── Order Repository ─────────────────────────────────────────────────────
pub struct PgOrderRepository { pub pool: PgPool }
impl PgOrderRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl OrderRepository for PgOrderRepository {
    async fn create_order(&self, org_id: Uuid, restaurant_id: Uuid, _table: Option<&str>, _opened_by: Option<Uuid>, _notes: Option<&str>) -> AppResult<Order> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = "pending";
        let row = sqlx::query("INSERT INTO orders (id, organization_id, restaurant_id, total, tax, service_charge, payment_method, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING *")
            .bind(id).bind(org_id).bind(restaurant_id)
            .bind(Decimal::new(0,0)).bind(Decimal::new(0,0)).bind(Decimal::new(0,0))
            .bind("").bind(status).bind(now).bind(now)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Order {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
            table_number: None, opened_by: None, closed_by: None,
            subtotal: Decimal::new(0,0), tax: Decimal::new(0,0), 
            service_charge: Decimal::new(0,0), discount: Decimal::new(0,0), total: Decimal::new(0,0),
            payment_method: None,
            status: rms::order::model::OrderStatus::Pending,
            notes: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            items: Vec::new(), closed_at: None,
        })
    }

    async fn add_items(&self, order_id: Uuid, items: Vec<(Uuid, i32, Decimal, Decimal, Option<String>)>) -> AppResult<Vec<OrderItem>> {
        let mut out = Vec::new();
        for (item_id, qty, price, _, _) in items {
            let row = sqlx::query("INSERT INTO order_items (order_id, menu_item_id, quantity, price) VALUES ($1, $2, $3, $4) RETURNING *")
                .bind(order_id).bind(item_id).bind(qty).bind(price)
                .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            out.push(OrderItem {
                order_id: row.try_get("order_id").unwrap_or_default(),
                menu_item_id: row.try_get("menu_item_id").unwrap_or_default(),
                quantity: row.try_get("quantity").unwrap_or_default(),
                unit_price: row.try_get("price").unwrap_or_default(),
                line_total: price * Decimal::from(qty),
                notes: None,
            });
        }
        Ok(out)
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Order>> {
        let row_res = sqlx::query("SELECT * FROM orders WHERE id = $1 AND organization_id = $2")
            .bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => {
                let status_str: String = row.try_get("status").unwrap_or_default();
                let status = match status_str.as_str() {
                    "paid" => rms::order::model::OrderStatus::Paid,
                    "cancelled" => rms::order::model::OrderStatus::Cancelled,
                    _ => rms::order::model::OrderStatus::Pending, // mapped from order_status enum
                };
                Ok(Some(Order {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                    table_number: None, opened_by: None, closed_by: None,
                    subtotal: Decimal::new(0,0), tax: row.try_get("tax").unwrap_or_default(), 
                    service_charge: row.try_get("service_charge").unwrap_or_default(), 
                    discount: Decimal::new(0,0), total: row.try_get("total").unwrap_or_default(),
                    payment_method: row.try_get("payment_method").ok(),
                    status, notes: None,
                    created_at: row.try_get("created_at").unwrap_or_default(),
                    updated_at: row.try_get("updated_at").unwrap_or_default(),
                    items: Vec::new(), closed_at: None,
                }))
            },
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, _restaurant_id: Option<Uuid>, limit: i64, offset: i64) -> AppResult<(Vec<Order>, i64)> {
        let rows = sqlx::query("SELECT * FROM orders WHERE organization_id = $1 LIMIT $2 OFFSET $3")
            .bind(org_id).bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        let count = sqlx::query("SELECT COUNT(*) FROM orders WHERE organization_id = $1").bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        
        let mut orders = Vec::new();
        for row in rows {
            orders.push(Order {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                table_number: None, opened_by: None, closed_by: None,
                subtotal: Decimal::new(0,0), tax: row.try_get("tax").unwrap_or_default(), 
                service_charge: row.try_get("service_charge").unwrap_or_default(), 
                discount: Decimal::new(0,0), total: row.try_get("total").unwrap_or_default(),
                payment_method: row.try_get("payment_method").ok(),
                status: rms::order::model::OrderStatus::Pending, notes: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                items: Vec::new(), closed_at: None,
            });
        }
        Ok((orders, count.try_get(0).unwrap_or(0)))
    }

    async fn close_order(&self, id: Uuid, _sub: Decimal, tax: Decimal, sc: Decimal, _disc: Decimal, total: Decimal, pm: &str, _closed_by: Option<Uuid>) -> AppResult<Order> {
        let now = Utc::now();
        sqlx::query("UPDATE orders SET tax = $1, service_charge = $2, total = $3, payment_method = $4, status = 'paid', updated_at = $5 WHERE id = $6")
            .bind(tax).bind(sc).bind(total).bind(pm).bind(now).bind(id)
            .execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id, Uuid::new_v4()).await?.ok_or(AppError::NotFound("Missing".into()))
    }
    
    async fn cancel_order(&self, id: Uuid) -> AppResult<Order> {
        let now = Utc::now();
        sqlx::query("UPDATE orders SET status = 'cancelled', updated_at = $1 WHERE id = $2")
            .bind(now).bind(id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id, Uuid::new_v4()).await?.ok_or(AppError::NotFound("Missing".into()))
    }

    async fn daily_revenue(&self, org_id: Uuid, restaurant_id: Uuid, _date: chrono::NaiveDate) -> AppResult<Decimal> {
        let row = sqlx::query("SELECT SUM(total) FROM orders WHERE organization_id = $1 AND restaurant_id = $2 AND status = 'paid'")
            .bind(org_id).bind(restaurant_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(row.try_get(0).unwrap_or(Decimal::new(0,0)))
    }
}

// ── Inventory Repository ─────────────────────────────────────────────────
pub struct PgInventoryRepository { pub pool: PgPool }
impl PgInventoryRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl InventoryRepository for PgInventoryRepository {
    async fn create(&self, restaurant_id: Uuid, name: &str, _unit: &str, _reorder_level: Decimal, _cost: Decimal) -> AppResult<InventoryItem> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let row = sqlx::query("INSERT INTO inventory_items (id, restaurant_id, name, quantity, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(id).bind(restaurant_id).bind(name).bind(Decimal::new(0,0)).bind(now).bind(now)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(InventoryItem {
            id: row.try_get("id").unwrap_or_default(),
            restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            unit: "unit".into(), reorder_level: Decimal::new(0,0), cost_per_unit: Decimal::new(0,0),
            quantity: row.try_get("quantity").unwrap_or_default(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
        deleted_at: None,
        })
    }

    async fn find_by_id(&self, id: Uuid) -> AppResult<Option<InventoryItem>> {
        let row_res = sqlx::query("SELECT * FROM inventory_items WHERE id = $1")
            .bind(id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => Ok(Some(InventoryItem {
                id: row.try_get("id").unwrap_or_default(),
                restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                unit: "unit".into(), reorder_level: Decimal::new(0,0), cost_per_unit: Decimal::new(0,0),
                quantity: row.try_get("quantity").unwrap_or_default(),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: None,
            })),
            None => Ok(None)
        }
    }
    async fn find_by_restaurant(&self, restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>> {
        let rows = sqlx::query("SELECT * FROM inventory_items WHERE restaurant_id = $1").bind(restaurant_id).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        let mut items = Vec::new();
        for row in rows {
            items.push(InventoryItem {
                id: row.try_get("id").unwrap_or_default(),
                restaurant_id: row.try_get("restaurant_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                unit: "unit".into(), reorder_level: Decimal::new(0,0), cost_per_unit: Decimal::new(0,0),
                quantity: row.try_get("quantity").unwrap_or_default(),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: None,
            });
        }
        Ok(items)
    }
    async fn adjust_quantity(&self, id: Uuid, delta: Decimal) -> AppResult<InventoryItem> {
        let now = Utc::now();
        sqlx::query("UPDATE inventory_items SET quantity = quantity + $1, updated_at = $2 WHERE id = $3")
            .bind(delta).bind(now).bind(id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id).await?.ok_or(AppError::NotFound("Item not found".into()))
    }
    async fn find_low_stock(&self, _restaurant_id: Uuid) -> AppResult<Vec<InventoryItem>> {
        Ok(Vec::new())
    }
}

// ── Stock Repository ─────────────────────────────────────────────────────
pub struct PgStockRepository { pub pool: PgPool }
impl PgStockRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl StockRepository for PgStockRepository {
    async fn record(&self, item_id: Uuid, qty: Decimal, m_type: MovementType, _ref_type: Option<String>, _ref_id: Option<Uuid>, _notes: Option<String>, _rec_by: Option<Uuid>) -> AppResult<StockMovement> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let type_str = match m_type { MovementType::In => "in", MovementType::Out => "out", MovementType::Adjustment => "adjustment", MovementType::Waste => "waste" };
        let row = sqlx::query("INSERT INTO stock_movements (id, inventory_item_id, quantity, movement_type, created_at) VALUES ($1, $2, $3, $4, $5) RETURNING *")
            .bind(id).bind(item_id).bind(qty).bind(type_str).bind(now)
            .fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(StockMovement {
            id: row.try_get("id").unwrap_or_default(),
            inventory_item_id: row.try_get("inventory_item_id").unwrap_or_default(),
            quantity: row.try_get("quantity").unwrap_or_default(),
            movement_type: m_type,
            reference_type: None, reference_id: None, notes: None, recorded_by: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
        })
    }
    async fn find_by_item(&self, _item_id: Uuid, _limit: i64, _offset: i64) -> AppResult<Vec<StockMovement>> {
        Ok(Vec::new())
    }
    async fn waste_total(&self, _restaurant_id: Uuid, _from: chrono::NaiveDate, _to: chrono::NaiveDate) -> AppResult<Decimal> {
        Ok(Decimal::new(0,0))
    }
}



