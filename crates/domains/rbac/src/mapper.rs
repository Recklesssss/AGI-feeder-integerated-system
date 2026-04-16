use super::model::Permission;
use super::error::RbacError;

/// Maps UPPER_SNAKE_CASE DB permission keys → Permission enum variants.
impl TryFrom<String> for Permission {
    type Error = RbacError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "VIEW_USERS"           => Ok(Permission::ViewUsers),
            "CREATE_USER"          => Ok(Permission::CreateUser),
            "DELETE_USER"          => Ok(Permission::DeleteUser),

            "CREATE_PAYMENT"       => Ok(Permission::CreatePayment),
            "VIEW_FINANCE"         => Ok(Permission::ViewFinance),
            "VIEW_REPORTS"         => Ok(Permission::ViewReports),

            "CREATE_PROPERTY"      => Ok(Permission::CreateProperty),
            "VIEW_PROPERTY"        => Ok(Permission::ViewProperty),
            "MANAGE_PROPERTY"      => Ok(Permission::ManageProperty),

            "CREATE_LEASE"         => Ok(Permission::CreateLease),
            "VIEW_LEASE"           => Ok(Permission::ViewLease),
            "MANAGE_MAINTENANCE"   => Ok(Permission::ManageMaintenance),

            "CREATE_LISTING"       => Ok(Permission::CreateListing),
            "VIEW_LISTING"         => Ok(Permission::ViewListing),

            "CREATE_DEAL"          => Ok(Permission::CreateDeal),
            "VIEW_DEAL"            => Ok(Permission::ViewDeal),
            "MANAGE_COMMISSION"    => Ok(Permission::ManageCommission),

            "CREATE_ORDER"         => Ok(Permission::CreateOrder),
            "VIEW_ORDER"           => Ok(Permission::ViewOrder),
            "MANAGE_MENU"          => Ok(Permission::ManageMenu),
            "MANAGE_INVENTORY"     => Ok(Permission::ManageInventory),
            "DAILY_CLOSING"        => Ok(Permission::DailyClosing),

            other => Err(RbacError::PermissionNotFound(
                format!("Unknown permission key: {other}"),
            )),
        }
    }
}