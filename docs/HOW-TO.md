# How-To Guides

This guide offers tactical, step-by-step instructions for specific edge cases within the UPRRMS backend logic plane.

---

## How to Establish Zero-Cost RBAC in a New Domain

In UPRRMS, we maintain a hard security perimeter: Handlers are not allowed to inspect their own permissions manually. We rely on the core `shared::extractors::AdminOnly` and `shared::extractors::CurrentUser` structs.

If you are developing a new endpoint (e.g., in the PMS domain to delete a Lease) that requires admin control, here is how you enforce it rapidly:

### Step 1: Import the Extractor
Ensure you are accessing our zero-cost boundary from the `shared` module, NOT the local logic.

```rust
use shared::extractors::AdminOnly;
use cores::AppResult;
use axum::{extract::Path, Json};
```

### Step 2: Inject the Guard Parameter
Axum parses request parameters serially. Insert the `AdminOnly` extractor as the *first* argument in your handler configuration. 

```rust
pub async fn terminate_lease(
    _admin_guard: AdminOnly, // Pre-flight check halts here on error!
    Path(lease_id): Path<Uuid>,
) -> AppResult<Json<Value>> {
    
    // Safety: If execution reaches here, we are 100% mathematically proven
    // that the executor carries a valid JWT mapped to the "admin" role.
    
    Ok(Json(serde_json::json!({
        "message": "Lease terminated securely."
    })))
}
```

By requesting `AdminOnly`, the global JWT middleware injects the `Claims`, and the extractor tests for `role == "admin"`. If false, a 403 Forbidden halts processing instantly.

---

## How to Initialize Sub-Domain SQLx Repositories

When creating a new bounded context (like PMS Property Maintenance), we never query raw database pools directly within the Domain crate. We rely on Dependency Injection (DI) through the `infra` layer. 

### Step 1: Define the Trait
In `domains/pms/src/maintenance/repository.rs`:
```rust
use axum::async_trait;

#[async_trait]
pub trait MaintenanceRepository: Send + Sync {
    async fn fetch_open_tickets(&self) -> Result<Vec<Ticket>, SqlxError>;
}
```

### Step 2: Implement the Postgres Logic in Infra
Move to the `infra` workspace crate. Let's bind it. In `infra/src/postgres_maintenance.rs`:
```rust
use sqlx::PgPool;
use pms::maintenance::repository::MaintenanceRepository;

pub struct PgMaintenanceRepository {
    pub db: PgPool,
}

#[async_trait]
impl MaintenanceRepository for PgMaintenanceRepository {
    async fn fetch_open_tickets(&self) -> Result<Vec<Ticket>, SqlxError> {
        sqlx::query_as!(Ticket, "SELECT * FROM maintenance WHERE status != 'closed'")
            .fetch_all(&self.db)
            .await
    }
}
```

### Step 3: Global Bootstrap
Finally, wire it in `app/src/bootstrap.rs`:
```rust
let maintenance_service = Arc::new(MaintenanceService::new(
    Arc::new(PgMaintenanceRepository { db: pool.clone() })
));
// Insert into AppState...
```
You have successfully separated infrastructure concerns from core business domains!
