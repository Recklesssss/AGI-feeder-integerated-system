use serde::Deserialize;
use uuid::Uuid;
use crate::client::model::ClientType;

#[derive(Debug, Deserialize)]
pub struct CreateClientDto {
    pub org_id:      Uuid,
    pub name:        String,
    pub email:       Option<String>,
    pub phone:       Option<String>,
    pub client_type: ClientType,
    pub source:      Option<String>,
}

/// Re-export Client as the response DTO.
/// Handlers return Json<ClientResponseDto> where ClientResponseDto = Client.
pub use crate::client::model::Client as ClientResponseDto;