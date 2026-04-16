# Tutorials: From Zero to UPRRMS in 5 Minutes

Welcome to the UPRRMS platform. We built this enterprise operating system specifically to power multi-tenant real estate pipelines, massive property property portfolios, and restaurant chains out of the box. 

This short tutorial will take you from a freshly cloned project to executing your very first authenticated financial ledger request.

---

## 1. Environment Setup

First, ensure you have Rust 1.75+ and PostgreSQL running on your machine. You will also need the `sqlx` tracking CLI.

```bash
cargo install sqlx-cli
```

Clone the unified workspace:

```bash
git clone https://github.com/your-org/uprrms.git
cd uprrms
cp .env.example .env
```

Ensure your `.env` contains a valid connection string:
`DATABASE_URL=postgres://postgres:password@localhost/uprrms`

---

## 2. Initialize the Immutable Ledger

UPRRMS depends on strict schema migrations to track assets. Execute the provided migration sequence to bind your local PostgreSQL instance to the architectural boundaries:

```bash
sqlx database setup
```

*This bridges the database mappings required by `assets`, `users`, `finance`, and `rems`.*

---

## 3. Ignite the Platform Engine

Start the overarching API router. Under the hood, this will bootstrap the database connection pool (`PgPool`) and inject it into all bounded contexts.

```bash
cargo run --bin app
```

If successful, `tracing` will output:
`[INFO] Starting AGI Enterprise Platform on 0.0.0.0:3000`

Verify your server connection in another terminal:

```bash
curl http://localhost:3000/health
```

Expected output:
```json
{
  "api": "AGI Enterprise Platform",
  "database": "ok",
  "status": "ok"
}
```

---

## 4. Become an Administrator

Because UPRRMS operates on a rigid zero-cost RBAC extraction boundary, you cannot view ledger states or property portfolios without an identity token.

### Step 4A: Register a Tenant
Let's create the root organization (Holding Company) and register our initial administrator.

```bash
curl -X POST http://localhost:3000/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "ceo@holdingcompany.com",
    "password": "super-secure-pass",
    "full_name": "Chief Executive",
    "organization_id": "00000000-0000-0000-0000-000000000000"
  }'
```

### Step 4B: Obtain Your Security Credentials

Now log in to generate an active JWT:

```bash
curl -X POST http://localhost:3000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "ceo@holdingcompany.com",
    "password": "super-secure-pass"
  }'
```

Capture the `access_token` returned in the payload. It holds your embedded permissions to traverse the system!

---

## 5. Explore the Network

With your token, you can now interrogate the secure endpoints! Let's ping the RBAC registry to see what powers we hold.

Replace `YOUR_JWT_HERE` and `YOUR_USER_ID` with the response targets from Step 4.

```bash
curl http://localhost:3000/api/v1/admin/rbac/{YOUR_USER_ID}/permissions \
  -H "Authorization: Bearer YOUR_JWT_HERE"
```

## Wrap Up

You executed database setups, fired up a massive multi-tenant multi-crate application, secured a session passing Argon2 encryption constraints, and tested a protected RBAC boundary via Axum extraction layers.

Head over to the [How-To Guides](./HOW-TO.md) to explore assigning sub-tenants to properties or bridging restaurant point-of-sale requests into the central ledger!
