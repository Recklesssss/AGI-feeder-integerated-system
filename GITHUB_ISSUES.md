# GitHub Issue Backlog: System Architecture Improvements

The following structural improvements have been identified against the DBML target state. You can copy and paste the markdown below directly into the GitHub "New Issue" modal. 

---

## 🏗️ Issue 1: Implement Core Integrity & Global Soft-Deletions
**Labels:** `enhancement`, `database`, `priority: high`
**Milestone:** Sprint 1: Security & Stability (Target Resolution: End of Week)

### Context & Problem Statement
Our current database migrations rely on a simple string-based `status = 'deleted'` mechanism, missing the chronological metadata of when a deletion occurred. To meet enterprise integrity standards defined in our DBML specification, we must enforce a physical `deleted_at TIMESTAMPTZ` column across the entire platform schema.

### Task List
- [ ] Write SQL `ALTER TABLE` migrations connecting `deleted_at TIMESTAMPTZ` to `organizations`, `users`, `assets`, `accounts`, `invoices`, `listings`, and `clients`.
- [ ] Update Rust `model.rs` files for all affected domains to include `pub deleted_at: Option<DateTime<Utc>>`.
- [ ] Update `find_all` and `get_by_*` repository SQL trait queries to append `WHERE deleted_at IS NULL`.

---

## 🔒 Issue 2: Establish the Complete RBAC Paradigm
**Labels:** `security`, `database`, `priority: high`
**Milestone:** Sprint 1: Security & Stability (Target Resolution: End of Week)

### Context & Problem Statement
Right now, our system maps permissions directly to users inside a `user_permissions` table. This creates massive data redundancy. The target DBML requires a scalable Group/Role architecture (`roles`, `permissions`, `role_permissions`, and `user_roles`). 

### Task List
- [ ] Draft `008_create_advanced_rbac.sql` schema connecting nested role foreign constraints.
- [ ] Migrate existing data from current flat constraints into the new relational layout.
- [ ] Update `rbac/repository.rs` to fetch aggregated permissions by `JOIN`ing `roles` and `role_permissions` instead of querying user vectors directly.

---

## 👁️ Issue 3: Implement Global Immutable Audit Trail
**Labels:** `feature`, `compliance`, `priority: medium`
**Milestone:** Sprint 2: Platform Automation (Target Resolution: Next Week)

### Context & Problem Statement
Enterprise financial systems require strict historical auditing of who changed what, and when. While we trace HTTP endpoints, we do not have an immutable ledger for data mutations.

### Task List
- [ ] Implement `009_create_audit.sql` table targeting `(user_id, organization_id, action, entity, entity_id, metadata JSONB)`.
- [ ] Build the `audit` domain crate with strict zero-deletion repository bindings.
- [ ] Inject audit interceptors into the `finance`, `assets`, and `users` `PUT`/`POST`/`DELETE` domain handlers.

---

## 🚀 Issue 4: Construct Full Operational Domain Schemas (PMS, RMS, REMS)
**Labels:** `feature`, `operations`, `priority: huge`
**Milestone:** Sprint 3: The Operating System (Target Resolution: End of Month)

### Context & Problem Statement
We have successfully stubbed the operational routes for Property Management (PMS), Restaurants (RMS), and Brokerages (REMS) at the API level, allowing frontend scaling. However, the deep database schemas running rentals, stock movements, and fractional commission payouts do not yet exist physically.

### Task List
- [ ] **PMS Integration**: Write `010_pms.sql` mapping `properties` -> `assets` (unique constraint), cascaded with `units`, `tenants`, `leases`, and `maintenance`.
- [ ] **RMS Integration**: Write `011_rms.sql` supporting numeric stock movements, POS menus, concurrent orders, and menu cross-referencing.
- [ ] **REMS Integration**: Write `012_rems_deals.sql` binding active clients to listings under deep `deal_value` and agent `commissions` ledgers.
- [ ] Generate Rust models mapping down to the newly deployed schema.
