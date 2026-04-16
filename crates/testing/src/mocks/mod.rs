//! Mock repository implementations for unit testing domain services
//! without requiring a live database connection.

pub mod audit;

use uuid::Uuid;

/// Generate a random Uuid for tests.
pub fn random_id() -> Uuid {
    Uuid::new_v4()
}

/// Generate a fixed org_id for consistent testing.
pub fn test_org_id() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap()
}

/// Generate a fixed user_id for consistent testing.
pub fn test_user_id() -> Uuid {
    Uuid::parse_str("00000000-0000-0000-0000-000000000002").unwrap()
}
