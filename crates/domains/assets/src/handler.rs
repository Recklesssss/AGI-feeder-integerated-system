use axum::{extract::{State, Path}, Json};
use std::sync::Arc;
use uuid::Uuid;
use serde::Deserialize;

use cores::AppResult;
use super::service::AssetService;
use super::model::AssetType;

#[derive(Debug, Deserialize)]
pub struct CreateAssetDto {
    pub organization_id: Uuid,
    pub asset_type: String,
    pub name: String,
}

#[derive(Debug, serde::Serialize)]
pub struct AssetResponseDto {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub asset_type: String,
    pub name: String,
    pub status: String,
}

fn parse_asset_type(s: &str) -> cores::AppResult<AssetType> {
    match s.to_uppercase().as_str() {
        "PROPERTY"   => Ok(AssetType::Property),
        "UNIT"       => Ok(AssetType::Unit),
        "RESTAURANT" => Ok(AssetType::Restaurant),
        "LISTING"    => Ok(AssetType::Listing),
        _ => Err(cores::AppError::Validation(format!("Unknown asset type: {s}"))),
    }
}

pub async fn create_asset_handler(
    State(svc): State<Arc<AssetService>>,
    Json(payload): Json<CreateAssetDto>,
) -> AppResult<Json<AssetResponseDto>> {
    let asset_type = parse_asset_type(&payload.asset_type)?;
    let asset = svc.create_asset(payload.organization_id, asset_type, payload.name).await?;
    Ok(Json(AssetResponseDto {
        id:              asset.id,
        organization_id: asset.organization_id,
        asset_type:      format!("{:?}", asset.asset_type),
        name:            asset.name,
        status:          format!("{:?}", asset.status),
    }))
}

pub async fn list_assets_handler(
    State(svc): State<Arc<AssetService>>,
    Path(org_id): Path<Uuid>,
) -> AppResult<Json<Vec<AssetResponseDto>>> {
    let assets = svc.list_assets(org_id).await?;
    let response = assets.into_iter().map(|a| AssetResponseDto {
        id:              a.id,
        organization_id: a.organization_id,
        asset_type:      format!("{:?}", a.asset_type),
        name:            a.name,
        status:          format!("{:?}", a.status),
    }).collect();
    Ok(Json(response))
}