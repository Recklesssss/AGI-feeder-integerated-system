// JWT Bearer validation logic lives in the parent middleware.rs (crate::middleware::jwt_auth).
// This file is reserved for fine-grained per-route auth guards (e.g., API key auth).
// T-01: See crate::middleware::jwt_auth for the primary implementation.
