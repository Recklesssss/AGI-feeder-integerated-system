use sqlx::{PgPool, Row};
use uuid::Uuid;
use async_trait::async_trait;

use cores::AppResult;
use rems::client::{
    model::{Client, ClientType},
    repository::ClientRepository,
};

pub struct PgClientRepository {
    pub db: PgPool,
}

fn client_type_from_str(s: &str) -> ClientType {
    match s {
        "seller" => ClientType::Seller,
        "lessee" => ClientType::Lessee,
        "lessor" => ClientType::Lessor,
        _        => ClientType::Buyer,
    }
}

fn map_client(row: &sqlx::postgres::PgRow) -> Result<Client, sqlx::Error> {
    let ct_str: String = row.try_get("client_type")?;
    Ok(Client {
        id:              row.try_get("id")?,
        organization_id: row.try_get("organization_id")?,
        name:            row.try_get("name")?,
        email:           row.try_get("email")?,
        phone:           row.try_get("phone")?,
        client_type:     client_type_from_str(&ct_str),
        source:          row.try_get("source")?,
        created_at:      row.try_get("created_at")?,
        updated_at:      row.try_get("updated_at")?,
        deleted_at:      row.try_get("deleted_at")?,
    })
}

const SELECT_COLS: &str =
    "id, organization_id, name, email, phone, client_type, source, created_at, updated_at, deleted_at";

#[async_trait]
impl ClientRepository for PgClientRepository {
    async fn create(
        &self, org_id: Uuid, name: &str, email: Option<&str>,
        phone: Option<&str>, client_type: ClientType, source: Option<&str>,
    ) -> AppResult<Client> {
        let row = sqlx::query(&format!(
            "INSERT INTO clients (organization_id, name, email, phone, client_type, source)
             VALUES ($1, $2, $3, $4, $5, $6)
             RETURNING {SELECT_COLS}",
        ))
        .bind(org_id)
        .bind(name)
        .bind(email)
        .bind(phone)
        .bind(client_type.as_str())
        .bind(source)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        map_client(&row).map_err(cores::AppError::from)
    }

    async fn find_by_id(&self, id: Uuid, org_id: Uuid) -> AppResult<Option<Client>> {
        let row = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM clients
             WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL",
        ))
        .bind(id)
        .bind(org_id)
        .fetch_optional(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        match row {
            Some(r) => Ok(Some(map_client(&r).map_err(cores::AppError::from)?)),
            None    => Ok(None),
        }
    }

    async fn find_all(
        &self, org_id: Uuid, limit: i64, offset: i64,
    ) -> AppResult<(Vec<Client>, i64)> {
        let rows = sqlx::query(&format!(
            "SELECT {SELECT_COLS} FROM clients
             WHERE organization_id = $1 AND deleted_at IS NULL
             ORDER BY created_at DESC LIMIT $2 OFFSET $3",
        ))
        .bind(org_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.db)
        .await
        .map_err(cores::AppError::from)?;

        let total: i64 = sqlx::query(
            "SELECT COUNT(*) AS count FROM clients WHERE organization_id = $1 AND deleted_at IS NULL",
        )
        .bind(org_id)
        .fetch_one(&self.db)
        .await
        .map_err(cores::AppError::from)?
        .get("count");

        let clients: AppResult<Vec<Client>> = rows
            .iter()
            .map(|r| map_client(r).map_err(cores::AppError::from))
            .collect();

        Ok((clients?, total))
    }

    async fn soft_delete(&self, id: Uuid, org_id: Uuid) -> AppResult<()> {
        sqlx::query(
            "UPDATE clients SET deleted_at = NOW(), updated_at = NOW()
             WHERE id = $1 AND organization_id = $2 AND deleted_at IS NULL",
        )
        .bind(id)
        .bind(org_id)
        .execute(&self.db)
        .await
        .map_err(cores::AppError::from)?;
        Ok(())
    }
}
