use uuid::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateAssetDto {
    pub organization_id: Uuid,
    pub asset_type: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateAssetStatusDto {
    pub status: String,
}

#[derive(Serialize)]
pub struct AssetResponseDto {
    pub id: Uuid,
    pub organization_id: Uuid,
    pub asset_type: String,
    pub name: String,
    pub status: String,
}