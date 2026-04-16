use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct CreateAccountDto {
    pub org_id:       Uuid,
    pub name:         String,
    pub account_type: String,
}