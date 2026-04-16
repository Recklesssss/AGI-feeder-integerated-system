use std::collections::HashSet;
use uuid::Uuid;

/// All permission keys that can be assigned to a user.
/// String conversion (TryFrom<String>) lives in mapper.rs to avoid duplicate impls.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Permission {
    ViewUsers,
    CreateUser,
    DeleteUser,

    CreatePayment,
    ViewFinance,
    ViewReports,

    CreateProperty,
    ViewProperty,
    ManageProperty,

    CreateLease,
    ViewLease,
    ManageMaintenance,

    CreateListing,
    ViewListing,

    CreateDeal,
    ViewDeal,
    ManageCommission,

    CreateOrder,
    ViewOrder,
    ManageMenu,
    ManageInventory,
    DailyClosing,
}

/// Authenticated user context — built by RbacService at request time.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub id: Uuid,
    pub permissions: HashSet<Permission>,
}

/// Advanced RBAC Models for Management

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Role {
    pub id:              Uuid,
    pub organization_id: Uuid,
    pub name:            String,
    pub description:     Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}