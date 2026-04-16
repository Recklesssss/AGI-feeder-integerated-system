use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;
use chrono::Utc;

use cores::{AppError, AppResult};
use assets::{
    model::{Asset, AssetStatus, AssetType},
    repository::AssetRepository,
};

pub struct PgAssetRepository {
    pub db: PgPool,
}

// ── private helpers ────────────────────────────────────────────────────────

fn asset_type_to_str(t: &AssetType) -> &'static str {
    match t {
        AssetType::Property   => "PROPERTY",
        AssetType::Unit       => "UNIT",
        AssetType::Restaurant => "RESTAURANT",
        AssetType::Listing    => "LISTING",
    }
}

fn asset_type_from_str(s: &str) -> AssetType {
    match s {
        "UNIT"       => AssetType::Unit,
        "RESTAURANT" => AssetType::Restaurant,
        "LISTING"    => AssetType::Listing,
        _            => AssetType::Property,
    }
}

fn asset_status_to_str(s: &AssetStatus) -> &'static str {
    match s {
        AssetStatus::Active   => "ACTIVE",
        AssetStatus::Inactive => "INACTIVE",
        AssetStatus::Archived => "ARCHIVED",
    }
}

fn asset_status_from_str(s: &str) -> AssetStatus {
    match s {
        "INACTIVE" => AssetStatus::Inactive,
        "ARCHIVED" => AssetStatus::Archived,
        _          => AssetStatus::Active,
    }
}

fn map_asset(row: &sqlx::postgres::PgRow) -> Result<Asset, sqlx::Error> {
    let asset_type_str: String = row.try_get("asset_type")?;
    let status_str: String     = row.try_get("status")?;
    Ok(Asset {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        asset_type:      asset_type_from_str(&asset_type_str),
        name:            row.try_get("name")?,
        status:          asset_status_from_str(&status_str),
        created_at:      row.try_get("created_at")?,
        updated_at:      row.try_get("updated_at")?,
    })
}

// ── trait implementation ───────────────────────────────────────────────────

#[async_trait]
impl AssetRepository for PgAssetRepository {
    async fn create_asset(&self, asset: Asset) -> AppResult<Asset> {
        sqlx::query(
            "INSERT INTO assets (id, organization_id, asset_type, name, status, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)",
        )
        .bind(asset.id)
        .bind(asset.organization_id)
        .bind(asset_type_to_str(&asset.asset_type))
        .bind(&asset.name)
        .bind(asset_status_to_str(&asset.status))
        .bind(asset.created_at)
        .bind(asset.updated_at)
        .execute(&self.db)
        .await
        .map_err(AppError::from)?;

        Ok(asset)
    }

    async fn get_asset_by_id(&self, id: Uuid) -> AppResult<Option<Asset>> {
        let row = sqlx::query(
            "SELECT id, organization_id, asset_type, name, status, created_at, updated_at
             FROM assets WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
        .map_err(AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_asset(&r).map_err(AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn list_assets(&self, org_id: Uuid) -> AppResult<Vec<Asset>> {
        let rows = sqlx::query(
            "SELECT id, organization_id, asset_type, name, status, created_at, updated_at
             FROM assets WHERE organization_id = $1",
        )
        .bind(org_id)
        .fetch_all(&self.db)
        .await
        .map_err(AppError::from)?;

        rows.iter()
            .map(|r| map_asset(r).map_err(AppError::from))
            .collect()
    }

    async fn update_status(&self, id: Uuid, status: String) -> AppResult<()> {
        sqlx::query(
            "UPDATE assets SET status = $1, updated_at = $2 WHERE id = $3",
        )
        .bind(status)
        .bind(Utc::now())
        .bind(id)
        .execute(&self.db)
        .await
        .map_err(AppError::from)?;

        Ok(())
    }
}