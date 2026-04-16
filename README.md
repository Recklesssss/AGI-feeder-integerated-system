<div align="center">
  <img src="https://placehold.co/1200x400/0a0a0a/ffffff/png?text=UPRRMS+|+The+Business+OS&font=Montserrat" alt="UPRRMS Banner" />

  <br />
  <br />

  <h3 align="center">Unified Property, Real Estate, and Restaurant Management System</h3>

  <p align="center">
    <strong>A high-performance, strictly typed Business Operating System engineered in Rust.</strong>
    <br />
    Deploy an enterprise-grade holding company backend powering dynamic portfolios from a single, immutable financial ledger.
    <br />
    <br />
    <a href="https://github.com/your-org/uprrms/issues">Report Bug</a>
    ·
    <a href="https://github.com/your-org/uprrms/issues">Request Feature</a>
    ·
    <a href="./docs/TUTORIALS.md">Documentation</a>
  </p>

  <p align="center">
    <a href="https://github.com/your-org/uprrms/actions/workflows/ci.yml">
      <img src="https://img.shields.io/github/actions/workflow/status/your-org/uprrms/ci.yml?style=for-the-badge&logo=github&color=2ea44f" alt="Build Status" />
    </a>
    <a href="https://rust-lang.org">
      <img src="https://img.shields.io/badge/Rust-1.75+-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Version" />
    </a>
    <a href="https://github.com/your-org/uprrms/blob/main/LICENSE">
      <img src="https://img.shields.io/badge/License-MIT-000000?style=for-the-badge&color=0969da" alt="MIT License" />
    </a>
  </p>
</div>

---

## ⚡ Why UPRRMS?

Modern conglomerates running real estate pipelines, property rentals, and physical restaurant chains rely on deeply fragmented software. **UPRRMS eliminates horizontal fracturing.** It centralizes massive operational scale under **one core double-entry accounting engine.**

Built cleanly on `axum`, `sqlx`, and `tokio`, it guarantees zero-cost abstractions, data integrity, and extreme concurrency out of the box.

<table align="center" width="100%">
  <tr>
    <td width="50%">
      <h3>🏢 Massive Multi-Tenancy</h3>
      Run as a holding company. Command thousands of independent corporate entities via the exact same backend array, isolated entirely by structural row-level tenancy at the physical datastore query level.
    </td>
    <td width="50%">
      <h3>🛡️ Zero-Cost RBAC Extraction</h3>
      Highly secure Rust-based Extractor logic ensures unauthorized requests are rejected instantaneously at the network boundary, validating JWT Claims and roles before handlers even spin up.
    </td>
  </tr>
  <tr>
    <td width="50%">
      <h3>💰 Immutable Financial Ledger</h3>
      Every operational movement—from ordering a side of fries to signing a 10-year commercial lease—triggers a strict double-entry ledger event. <strong>Zero financial leaks. Complete auditing truth.</strong>
    </td>
    <td width="50%">
      <h3>👁️ Deep Telemetry</h3>
      Baked-in <code>tower-http</code> and <code>tracing-subscriber</code> frameworks provide localized, high-resolution JSON logs tracking the deepest execution times across all domain bounds.
    </td>
  </tr>
</table>

---

## 🧠 System Architecture

Instead of a catastrophic microservice web or a tangled monolith, **UPRRMS uses a strict Modular Monolithic design**. Domains are compiled as independent crates, injected dynamically via a top-level bootstrap configuration into a central `axum::Router`.

```mermaid
graph TD;
    Client[Web / Mobile Clients] --> API(Axum Router layer);
    
    subgraph System ["UPRRMS Workspace"]
      API --> App[app crate <br> Auth & Routing];
      App --> Shared[shared crate <br> RBAC & Config];
      
      subgraph Domains ["Bounded Context Crate Deployments"]
        App --> PMS[PMS <br> Property Management];
        App --> RMS[RMS <br> Restaurant Ops];
        App --> REMS[REMS <br> Real Estate Brokerage];
        
        PMS -. triggers .-> Finance[Finance <br> Ledger / Payments];
        RMS -. triggers .-> Finance;
        REMS -. triggers .-> Finance;
      end
      
      Finance --> Infra[(Infra Crate <br> sqlx PostgreSQL Adapters)];
      PMS --> Infra;
      RMS --> Infra;
      REMS --> Infra;
    end
```

---

## 🚀 Quick Start

Spin up the UPRRMS intelligence engine locally in under 60 seconds.

<details>
<summary><strong>▶ Click to expand installation instructions</strong></summary>

### 1. Prerequisites
- **Rust Toolchain** (v1.75+) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- **PostgreSQL Database** running natively or via Docker.
- **SQLx CLI** `cargo install sqlx-cli`

### 2. Environment Setup

```bash
git clone https://github.com/your-org/uprrms.git
cd uprrms

# Copy template configurations
cp .env.example .env

# Verify database connection string (e.g., postgres://postgres:pass@localhost:5432/uprrms)
nano .env 
```

### 3. Migrate & Ignite

```bash
# Execute physical database schema bindings
sqlx database setup

# Boot the API server on 0.0.0.0:3000
cargo run --bin app
```

> [!TIP]
> **Health Check Endpoint:** Test your boot sequence dynamically by reaching the health monitor!
> ```bash
> curl http://localhost:3000/health
> ```
> *Expected output: `{"api":"AGI Enterprise Platform","database":"ok","status":"ok"}`*

</details>

---

## 🧰 The Operational Domains

| Sub-Domain Crate | Responsibilities | Target Trajectory |
| :--- | :--- | :--- |
| **`assets`** | Foundational registry identity mapping physical properties and restaurants. | Live |
| **`finance`** | Strict double-entry ledger logic, invoice resolution, payment collection. | Live |
| **`users`** / **`rbac`** | Argon2 hashing, identity allocation, and granular permission handling. | Live |
| **`pms`** | Property Management: Tenants, complex leases, and maintenance workflows. | Pre-Alpha |
| **`rms`** | Restaurant Management: Advanced menu parsing, POS coordination, and inventory. | Pre-Alpha |
| **`rems`** | Brokerage pipeline tracking: Client onboarding, deal lifecycles, & commissions. | Pre-Alpha |

---

## 📖 Complete Documentation Framework

We utilize the Diátaxis documentation framework to support all engineering operations. Exploring the platform is simple:

- **[Tutorials](./docs/TUTORIALS.md):** A 5-minute "Getting Started" guide to clone, boot, and authenticate.
- **[How-To Guides](./docs/HOW-TO.md):** Step-by-step instructions for specific integrations (e.g., RBAC & Repositories).
- **[Explanation](./docs/EXPLANATION.md):** Deep dives into our Architectural decisions (Monoliths, Double-Entry systems).
- **[API Reference](./docs/REFERENCE.md):** The lookup grid mapping global extractors and trait signatures.

---

## 🔒 Security Posture

We maintain an uncompromising approach to backend security:
1. **Never pass ID-lookups in JSON bodies.** All endpoint resource resolutions require strict URL `Path` injection.
2. **Never check roles inside business logic.** Every secure endpoint requires an architectural constraint pre-flight via the `AdminOnly` or `CurrentUser` Axum Extractors.
3. **No direct database coupling.** All databases connect strictly through `Arc<dyn RepositoryLayer>` traits to prevent SQL injection and allow 100% mock testing coverage.

## 🤝 Contributing

We rely heavily on the extraordinary standard of the open-source Rust community. If you have recognized a performance loophole, an architectural deficiency, or want to deploy an entirely new Operational Domain (like Fleet Management), we want your PRs.

Review our full [CONTRIBUTING.md](./CONTRIBUTING.md) to understand strictly enforced **Conventional Commits** protocols and structural formatting.

---
<div align="center">
  <sub>Built with power by the AGI Enterprise Team.</sub>
</div>
