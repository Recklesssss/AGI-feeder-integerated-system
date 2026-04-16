use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{NaiveDate, Utc};

use cores::{AppError, AppResult};

use pms::property::{repository::PropertyRepository, model::Property};
use pms::unit::{repository::UnitRepository, model::Unit};
use pms::tenant::{repository::TenantRepository, model::Tenant};
use pms::lease::{repository::LeaseRepository, model::Lease};
use pms::maintenance::{repository::MaintenanceRepository, model::MaintenanceRequest};

// ── Property Repository ──────────────────────────────────────────────────
pub struct PgPropertyRepository { pub pool: PgPool }
impl PgPropertyRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl PropertyRepository for PgPropertyRepository {
    async fn create(&self, org_id: Uuid, asset_id: Uuid, address: &str, _city: Option<&str>, _country: Option<&str>) -> AppResult<Property> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let row = sqlx::query("INSERT INTO properties (id, organization_id, asset_id, address, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(id)
            .bind(org_id)
            .bind(asset_id)
            .bind(address)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Property {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            asset_id: row.try_get("asset_id").unwrap_or_default(),
            address: row.try_get("address").unwrap_or_default(), property_type: "residential".into(),
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Property>> {
        let row_res = sqlx::query("SELECT * FROM properties WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL")
            .bind(id)
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => Ok(Some(Property {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                address: row.try_get("address").unwrap_or_default(), property_type: "residential".into(),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            })),
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Property>, i64)> {
        let rows = sqlx::query("SELECT * FROM properties WHERE organization_id = $1 AND deleted_at IS NULL LIMIT $2 OFFSET $3")
            .bind(org_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        let mut props = Vec::new();
        for row in rows {
            props.push(Property {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                address: row.try_get("address").unwrap_or_default(), property_type: "residential".into(),
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            });
        }
        
        let count_row = sqlx::query("SELECT COUNT(*) FROM properties WHERE organization_id = $1 AND deleted_at IS NULL")
            .bind(org_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        let total: i64 = count_row.try_get(0).unwrap_or(0);
        Ok((props, total))
    }

    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        let now = Utc::now();
        sqlx::query("UPDATE properties SET deleted_at = $1 WHERE id = $2 AND organization_id = $3")
            .bind(now)
            .bind(id)
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }
}

// ── Unit Repository ──────────────────────────────────────────────────────
pub struct PgUnitRepository { pub pool: PgPool }
impl PgUnitRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl UnitRepository for PgUnitRepository {
    async fn create(&self, org_id: Uuid, property_id: Uuid, asset_id: Uuid, unit_number: &str, _floor: Option<i32>, _bedrooms: Option<i32>, _bathrooms: Option<i32>, _area_sqm: Option<Decimal>) -> AppResult<Unit> {
        let id = Uuid::new_v4();
        let status = "vacant";
        let now = Utc::now();
        
        let row = sqlx::query("INSERT INTO units (id, organization_id, property_id, asset_id, unit_number, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8) RETURNING *")
            .bind(id)
            .bind(org_id)
            .bind(property_id)
            .bind(asset_id)
            .bind(unit_number)
            .bind(status)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Unit {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            property_id: row.try_get("property_id").unwrap_or_default(),
            asset_id: row.try_get("asset_id").unwrap_or_default(),
            unit_number: row.try_get("unit_number").unwrap_or_default(),
            status: match row.try_get::<String, _>("status").unwrap_or_default().as_str()  { "occupied" => pms::unit::model::UnitStatus::Occupied, "maintenance" => pms::unit::model::UnitStatus::UnderMaintenance, _ => pms::unit::model::UnitStatus::Vacant },
            floor: None,
            bedrooms: None,
            bathrooms: None,
            area_sqm: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Unit>> {
        let row_res = sqlx::query("SELECT * FROM units WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL")
            .bind(id)
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => {
                let status_str: String = row.try_get("status").unwrap_or_default();
                let status = match status_str.as_str() {
                    "occupied" => pms::unit::model::UnitStatus::Occupied,
                    "maintenance" => pms::unit::model::UnitStatus::UnderMaintenance,
                    _ => pms::unit::model::UnitStatus::Vacant,
                };
                Ok(Some(Unit {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                property_id: row.try_get("property_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                unit_number: row.try_get("unit_number").unwrap_or_default(),
                status,
                floor: None, bedrooms: None, bathrooms: None, area_sqm: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            }))
            },
            None => Ok(None)
        }
    }

    async fn find_by_property(&self, property_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Unit>, i64)> {
        let rows = sqlx::query("SELECT * FROM units WHERE property_id = $1 AND deleted_at IS NULL LIMIT $2 OFFSET $3")
            .bind(property_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;

        let mut out = Vec::new();
        for row in rows {
            let status_str: String = row.try_get("status").unwrap_or_default();
            let status = match status_str.as_str() {
                "occupied" => pms::unit::model::UnitStatus::Occupied,
                "maintenance" => pms::unit::model::UnitStatus::UnderMaintenance,
                _ => pms::unit::model::UnitStatus::Vacant,
            };
            out.push(Unit {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                property_id: row.try_get("property_id").unwrap_or_default(),
                asset_id: row.try_get("asset_id").unwrap_or_default(),
                unit_number: row.try_get("unit_number").unwrap_or_default(),
                status,
                floor: None, bedrooms: None, bathrooms: None, area_sqm: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            });
        }
        
        let count_row = sqlx::query("SELECT COUNT(*) FROM units WHERE property_id = $1 AND deleted_at IS NULL")
            .bind(property_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok((out, count_row.try_get(0).unwrap_or(0)))
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Unit> {
        let now = Utc::now();
        let row = sqlx::query("UPDATE units SET status = $1, updated_at = $2 WHERE id = $3 RETURNING *")
            .bind(status)
            .bind(now)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Unit {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            property_id: row.try_get("property_id").unwrap_or_default(),
            asset_id: row.try_get("asset_id").unwrap_or_default(),
            unit_number: row.try_get("unit_number").unwrap_or_default(),
            status: match row.try_get::<String, _>("status").unwrap_or_default().as_str()  { "occupied" => pms::unit::model::UnitStatus::Occupied, "maintenance" => pms::unit::model::UnitStatus::UnderMaintenance, _ => pms::unit::model::UnitStatus::Vacant },
            floor: None, bedrooms: None, bathrooms: None, area_sqm: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE units SET deleted_at = $1 WHERE id = $2 AND organization_id = $3")
            .bind(Utc::now())
            .bind(id)
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }

    async fn count_vacant(&self, property_id: Uuid) -> AppResult<i64> {
        let row = sqlx::query("SELECT COUNT(*) FROM units WHERE property_id = $1 AND status = 'vacant' AND deleted_at IS NULL")
            .bind(property_id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(row.try_get(0).unwrap_or(0))
    }
}

// ── Tenant Repository ────────────────────────────────────────────────────
pub struct PgTenantRepository { pub pool: PgPool }
impl PgTenantRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl TenantRepository for PgTenantRepository {
    async fn create(&self, org_id: Uuid, name: &str, email: Option<&str>, _phone: Option<&str>, _national_id: Option<&str>) -> AppResult<Tenant> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let contact = email.unwrap_or("").to_string(); // simple mock for standard DBML contact
        
        let row = sqlx::query("INSERT INTO tenants (id, organization_id, name, contact, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6) RETURNING *")
            .bind(id)
            .bind(org_id)
            .bind(name)
            .bind(&contact)
            .bind(now)
            .bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok(Tenant {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            name: row.try_get("name").unwrap_or_default(),
            email: Some(row.try_get("contact").unwrap_or_default()),
            phone: None,
            national_id: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: row.try_get("deleted_at").unwrap_or_default(),
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Tenant>> {
        let row_res = sqlx::query("SELECT * FROM tenants WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL")
            .bind(id)
            .bind(org_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => Ok(Some(Tenant {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                email: Some(row.try_get("contact").unwrap_or_default()),
                phone: None, national_id: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            })),
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Tenant>, i64)> {
        let rows = sqlx::query("SELECT * FROM tenants WHERE organization_id = $1 AND deleted_at IS NULL LIMIT $2 OFFSET $3")
            .bind(org_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;

        let mut out = Vec::new();
        for row in rows {
            out.push(Tenant {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                name: row.try_get("name").unwrap_or_default(),
                email: Some(row.try_get("contact").unwrap_or_default()),
                phone: None, national_id: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: row.try_get("deleted_at").unwrap_or_default(),
            });
        }
        
        let count_row = sqlx::query("SELECT COUNT(*) FROM tenants WHERE organization_id = $1 AND deleted_at IS NULL")
            .bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        Ok((out, count_row.try_get(0).unwrap_or(0)))
    }

    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        sqlx::query("UPDATE tenants SET deleted_at = $1 WHERE id = $2 AND organization_id = $3")
            .bind(Utc::now())
            .bind(id)
            .bind(org_id)
            .execute(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
        Ok(())
    }
}

// ── Lease Repository ─────────────────────────────────────────────────────
pub struct PgLeaseRepository { pub pool: PgPool }
impl PgLeaseRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl LeaseRepository for PgLeaseRepository {
    async fn create(&self, org_id: Uuid, unit_id: Uuid, tenant_id: Uuid, rent: Decimal, security_deposit: Decimal, late_fee: Decimal, _billing_day: i32, start_date: NaiveDate, end_date: NaiveDate, _notes: Option<&str>) -> AppResult<Lease> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        // The lease_status enum might just be standard pms::lease::model::LeaseStatus::Active
        let status = "active";
        
        let row = sqlx::query("INSERT INTO leases (id, organization_id, unit_id, tenant_id, rent, security_deposit, late_fee, start_date, end_date, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12) RETURNING *")
            .bind(id).bind(org_id).bind(unit_id).bind(tenant_id)
            .bind(rent).bind(security_deposit).bind(late_fee)
            .bind(start_date).bind(end_date).bind(status)
            .bind(now).bind(now)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| AppError::DbError(e.to_string()))?;
            
        // Map string status to enum 
        let status_str: String = row.try_get("status").unwrap_or_default();
        let status_enum = match status_str.as_str() {
            "terminated" => pms::lease::model::LeaseStatus::Terminated,
            "expired" => pms::lease::model::LeaseStatus::Expired,
            _ => pms::lease::model::LeaseStatus::Active,
        };

        Ok(Lease {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            unit_id: row.try_get("unit_id").unwrap_or_default(),
            tenant_id: row.try_get("tenant_id").unwrap_or_default(),
            rent: row.try_get("rent").unwrap_or_default(),
            security_deposit: row.try_get("security_deposit").unwrap_or_default(),
            late_fee: row.try_get("late_fee").unwrap_or_default(),
            billing_day: 1, // mocked stub
            start_date: row.try_get("start_date").unwrap_or_default(),
            end_date: row.try_get("end_date").unwrap_or_default(),
            status: status_enum,
            notes: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: None,
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Lease>> {
        let row_res = sqlx::query("SELECT * FROM leases WHERE id = $1 AND organization_id = $2")
            .bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => {
                let status_str: String = row.try_get("status").unwrap_or_default();
                let status = match status_str.as_str() {
                    "terminated" => pms::lease::model::LeaseStatus::Terminated,
                    "expired" => pms::lease::model::LeaseStatus::Expired,
                    _ => pms::lease::model::LeaseStatus::Active,
                };
                Ok(Some(Lease {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    unit_id: row.try_get("unit_id").unwrap_or_default(),
                    tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                    rent: row.try_get("rent").unwrap_or_default(),
                    security_deposit: row.try_get("security_deposit").unwrap_or_default(),
                    late_fee: row.try_get("late_fee").unwrap_or_default(),
                    billing_day: 1,
                    start_date: row.try_get("start_date").unwrap_or_default(),
                    end_date: row.try_get("end_date").unwrap_or_default(),
                    status,
                    notes: None,
                    created_at: row.try_get("created_at").unwrap_or_default(),
                    updated_at: row.try_get("updated_at").unwrap_or_default(),
                    deleted_at: None,
                }))
            },
            None => Ok(None)
        }
    }

    async fn find_active_by_unit(&self, unit_id: Uuid) -> AppResult<Option<Lease>> {
        let row_res = sqlx::query("SELECT * FROM leases WHERE unit_id = $1 AND status = 'active' LIMIT 1")
            .bind(unit_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
            
        match row_res {
            Some(row) => {
                Ok(Some(Lease {
                    id: row.try_get("id").unwrap_or_default(),
                    organization_id: row.try_get("organization_id").unwrap_or_default(),
                    unit_id: row.try_get("unit_id").unwrap_or_default(),
                    tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                    rent: row.try_get("rent").unwrap_or_default(),
                    security_deposit: row.try_get("security_deposit").unwrap_or_default(),
                    late_fee: row.try_get("late_fee").unwrap_or_default(),
                    billing_day: 1,
                    start_date: row.try_get("start_date").unwrap_or_default(),
                    end_date: row.try_get("end_date").unwrap_or_default(),
                    status: pms::lease::model::LeaseStatus::Active,
                    notes: None,
                    created_at: row.try_get("created_at").unwrap_or_default(),
                    updated_at: row.try_get("updated_at").unwrap_or_default(),
                    deleted_at: None,
                }))
            },
            None => Ok(None)
        }
    }

    async fn find_all(&self, org_id: Uuid, limit: i64, offset: i64) -> AppResult<(Vec<Lease>, i64)> {
        let rows = sqlx::query("SELECT * FROM leases WHERE organization_id = $1 LIMIT $2 OFFSET $3")
            .bind(org_id).bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        
        let mut out = Vec::new();
        for row in rows {
            let status_str: String = row.try_get("status").unwrap_or_default();
            out.push(Lease {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                unit_id: row.try_get("unit_id").unwrap_or_default(),
                tenant_id: row.try_get("tenant_id").unwrap_or_default(),
                rent: row.try_get("rent").unwrap_or_default(),
                security_deposit: row.try_get("security_deposit").unwrap_or_default(),
                late_fee: row.try_get("late_fee").unwrap_or_default(),
                billing_day: 1,
                start_date: row.try_get("start_date").unwrap_or_default(),
                end_date: row.try_get("end_date").unwrap_or_default(),
                status: match status_str.as_str() {
                    "terminated" => pms::lease::model::LeaseStatus::Terminated,
                    "expired" => pms::lease::model::LeaseStatus::Expired,
                    _ => pms::lease::model::LeaseStatus::Active,
                },
                notes: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: None,
            });
        }
        let count_row = sqlx::query("SELECT COUNT(*) FROM leases WHERE organization_id = $1").bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok((out, count_row.try_get(0).unwrap_or(0)))
    }

    async fn update_status(&self, id: Uuid, status: &str) -> AppResult<Lease> {
        let now = Utc::now();
        let row = sqlx::query("UPDATE leases SET status = $1, updated_at = $2 WHERE id = $3 RETURNING *")
            .bind(status).bind(now).bind(id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id, row.try_get("organization_id").unwrap_or_default()).await?.ok_or(AppError::NotFound("Lease missing".into()))
    }

    async fn find_expiring_soon(&self, org_id: Uuid, _within_days: i32) -> AppResult<Vec<Lease>> {
        let (out, _) = self.find_all(org_id, 1000, 0).await?;
        Ok(out) // Simplified mapping
    }
}

// ── Maintenance Repository ───────────────────────────────────────────────
pub struct PgMaintenanceRepository { pub pool: PgPool }
impl PgMaintenanceRepository { pub fn new(pool: PgPool) -> Self { Self { pool } } }

#[async_trait]
impl MaintenanceRepository for PgMaintenanceRepository {
    async fn create(&self, org_id: Uuid, unit_id: Uuid, description: &str, _priority: &str, _reported_by: Option<Uuid>) -> AppResult<MaintenanceRequest> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let status = "pending";
        let row = sqlx::query("INSERT INTO maintenance_requests (id, organization_id, unit_id, description, status, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *")
            .bind(id).bind(org_id).bind(unit_id).bind(description).bind(status).bind(now).bind(now).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        
        Ok(MaintenanceRequest {
            id: row.try_get("id").unwrap_or_default(),
            organization_id: row.try_get("organization_id").unwrap_or_default(),
            unit_id: row.try_get("unit_id").unwrap_or_default(),
            description: row.try_get("description").unwrap_or_default(),
            priority: pms::maintenance::model::MaintenancePriority::Low, // stub
            status: pms::maintenance::model::MaintenanceStatus::Open,
            reported_by: None, assigned_to: None, estimated_cost: None, actual_cost: None, resolved_at: None,
            created_at: row.try_get("created_at").unwrap_or_default(),
            updated_at: row.try_get("updated_at").unwrap_or_default(),
            deleted_at: None,
        })
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<MaintenanceRequest>> {
        let row_res = sqlx::query("SELECT * FROM maintenance_requests WHERE id = $1 AND organization_id = $2")
            .bind(id).bind(org_id).fetch_optional(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        match row_res {
            Some(row) => Ok(Some(MaintenanceRequest {
                id: row.try_get("id").unwrap_or_default(),
                organization_id: row.try_get("organization_id").unwrap_or_default(),
                unit_id: row.try_get("unit_id").unwrap_or_default(),
                description: row.try_get("description").unwrap_or_default(),
                priority: pms::maintenance::model::MaintenancePriority::Low,
                status: pms::maintenance::model::MaintenanceStatus::Open,
                reported_by: None, assigned_to: None, estimated_cost: None, actual_cost: None, resolved_at: None,
                created_at: row.try_get("created_at").unwrap_or_default(),
                updated_at: row.try_get("updated_at").unwrap_or_default(),
                deleted_at: None,
            })),
            None => Ok(None)
        }
    }
    async fn find_all(&self, org_id: Uuid, _limit: i64, _offset: i64) -> AppResult<(Vec<MaintenanceRequest>, i64)> {
        let count = sqlx::query("SELECT COUNT(*) FROM maintenance_requests WHERE organization_id = $1").bind(org_id).fetch_one(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        Ok((Vec::new(), count.try_get(0).unwrap_or(0)))
    }
    async fn assign(&self, id: Uuid, _user_id: Uuid) -> AppResult<MaintenanceRequest> {
        self.find_by_id(id, Uuid::new_v4()).await?.ok_or(AppError::NotFound("Not found".into()))
    }
    async fn update_status(&self, id: Uuid, status: &str, _actual_cost: Option<Decimal>) -> AppResult<MaintenanceRequest> {
        let now = Utc::now();
        sqlx::query("UPDATE maintenance_requests SET status = $1, updated_at = $2 WHERE id = $3 RETURNING *")
            .bind(status).bind(now).bind(id).execute(&self.pool).await.map_err(|e| AppError::DbError(e.to_string()))?;
        self.find_by_id(id, Uuid::new_v4()).await?.ok_or(AppError::NotFound("Not found".into()))
    }
}



