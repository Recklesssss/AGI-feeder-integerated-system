# API & Technical Reference

This document serves as the absolute technical reference covering our foundational schemas, global extractors, and endpoints.

---

## Global Extractor Reference

These structs orchestrate our entire role and identity checking matrices. Found in the `shared::extractors` crate.

| Extractor Structure | Requirement | HTTP Response on Failure | Internal Payload |
| :--- | :--- | :--- | :--- |
| `CurrentUser` | Requires a valid `Authorization: Bearer <JWT>` header | `401 Unauthorized` | `sub` (user UUID), `org_id` |
| `AdminOnly` | Requires JWT AND `role == "admin"` embedded in JWT Claims | `403 Forbidden` | Complete Claims object |

---

## Base API Reference

> Note: All domain routes enforce `jwt_auth` verification via the `app::middleware` globally, except for the paths mapped under `/auth/`.

### Identity Subsystems

#### `POST /api/v1/auth/login`
Validates Argon2 password hashes against database records.
**Body Requirements:**
| Parameter | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `email` | `String` | Yes | Identity marker |
| `password` | `String` | Yes | Raw string payload |

**Returns:** 
| Payload | Type | Description |
| :--- | :--- | :--- |
| `access_token` | `String` | Signed JWT bearing Role & Organization |
| `refresh_token` | `String` | 7-day TTL extension payload |

---

#### `GET /api/v1/admin/rbac/{user_id}/permissions`
Interrogates the permission matrices attached to dynamic sub-users.
**Header:** `Authorization: Bearer <Admin JWT>`
**Path Variables:**
| Parameter | Type | Description |
| :--- | :--- | :--- |
| `user_id` | `UUID v4` | Target user to interrogate. |

**Returns:**
| Payload | Type | Description |
| :--- | :--- | :--- |
| `permissions` | `[String]` | Array of approved permissions e.g., `["VIEW_FINANCE"]` |

---

### Internal Trait Interfaces

#### `AuthRepository` (Infrastructure Contract)
| Method Signature | Action Result | Concrete Adopter |
| :--- | :--- | :--- |
| `find_by_email(&str)` | `Ok(Option<AuthCredentials>)` | `PgAuthRepository` |
| `create_user(Uuid, Uuid, &str, &str)` | `Ok(AuthCredentials)` | `PgAuthRepository` |

#### `LedgerRepository` (Finance Contract)
| Method Signature | Action Result | Primary Business Use Case |
| :--- | :--- | :--- |
| `create_entry(...)` | `Ok(LedgerEntry)` | Booking revenue upon PMS renting |
| `get_account_balance(Uuid)` | `Ok(Decimal)` | Total cash position queries |
