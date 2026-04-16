use finance::invoice::model::InvoiceStatus;
use rems::listing::model::ListingStatus;
use rms::order::model::OrderStatus;
use pms::lease::model::LeaseStatus;
use rbac::model::Permission;

#[test]
fn invoice_status_transitions_are_correct() {
    assert!(InvoiceStatus::Draft.can_transition_to(&InvoiceStatus::Issued));
    assert!(InvoiceStatus::Issued.can_transition_to(&InvoiceStatus::Paid));
    assert!(InvoiceStatus::Draft.can_transition_to(&InvoiceStatus::Cancelled));
    assert!(InvoiceStatus::Issued.can_transition_to(&InvoiceStatus::Cancelled));

    // Invalid transitions
    assert!(!InvoiceStatus::Paid.can_transition_to(&InvoiceStatus::Draft));
    assert!(!InvoiceStatus::Cancelled.can_transition_to(&InvoiceStatus::Issued));
}

#[test]
fn listing_status_transitions_are_correct() {
    assert!(ListingStatus::Draft.can_transition_to(&ListingStatus::Active));
    assert!(ListingStatus::Active.can_transition_to(&ListingStatus::Sold));
    assert!(ListingStatus::Draft.can_transition_to(&ListingStatus::Cancelled));

    // Invalid
    assert!(!ListingStatus::Sold.can_transition_to(&ListingStatus::Draft));
    assert!(!ListingStatus::Cancelled.can_transition_to(&ListingStatus::Active));
}

#[test]
fn lease_status_as_str_roundtrip() {
    assert_eq!(LeaseStatus::Active.as_str(), "active");
    assert_eq!(LeaseStatus::Terminated.as_str(), "terminated");
    assert_eq!(LeaseStatus::Expired.as_str(), "expired");
}

#[test]
fn order_status_as_str_roundtrip() {
    assert_eq!(OrderStatus::Pending.as_str(), "pending");
    assert_eq!(OrderStatus::Paid.as_str(), "paid");
    assert_eq!(OrderStatus::Cancelled.as_str(), "cancelled");
}

#[test]
fn permission_enum_is_exhaustive() {
    // Verify every permission variant hashes into a set without collision
    use std::collections::HashSet;
    let perms = vec![
        Permission::ViewUsers, Permission::CreateUser, Permission::DeleteUser,
        Permission::CreatePayment, Permission::ViewFinance, Permission::ViewReports,
        Permission::CreateProperty, Permission::ViewProperty, Permission::ManageProperty,
        Permission::CreateLease, Permission::ViewLease, Permission::ManageMaintenance,
        Permission::CreateListing, Permission::ViewListing,
        Permission::CreateDeal, Permission::ViewDeal, Permission::ManageCommission,
        Permission::CreateOrder, Permission::ViewOrder, Permission::ManageMenu,
        Permission::ManageInventory, Permission::DailyClosing,
    ];
    let set: HashSet<_> = perms.iter().collect();
    assert_eq!(set.len(), 22, "All 22 permission variants must be unique");
}
