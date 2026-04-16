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