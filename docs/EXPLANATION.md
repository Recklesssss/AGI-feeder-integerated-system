# Explanation: UPRRMS Architecture Secrets

Why did we choose a modular monolith? Why do we use trait-bound Repository logic? How does the immutable ledger interact across restaurants and real estate simultaneously?

Here is a deep dive into our core architectural decision logs.

---

## 1. The Modular Monolithic Approach
**Why Rust workspaces? Why not microservices?**

We chose to split the project into 10+ crates (`app`, `cores`, `infra`, `shared`, `finance`, `pms`, `rms`...) rather than a giant flat codebase *or* individual microservice APIs over gRPC.

*   **Compilation Boundaries & Scale**: Cargo checks independent domains in parallel. If `users` is updated, `finance` doesn't need to rebuild. 
*   **Preventing Spaghetti State**: Rust enforces directed acyclic graphs (DAGs) for dependencies. `Finance` cannot depend on `Assets` if `Assets` depends on `Finance`. This strict adherence forces elegant domain-driven-design (DDD) boundaries out of the box. 
*   **Operational Velocity**: We run as a single `app` binary deployment. No Kubernetes sidecars, no intra-network latency penalties between PMS checking a Lease and Finance generating an Invoice. Everything executes in micro-seconds traversing memory instead of a network stack.

---

## 2. Inversion of Control (IoC) via the Infra Crate 

Notice that our business logic crates (`finance`, `users`, `pms`) do not mention `sqlx` or PostgreSQL. They only define data structs and `Repository` generic traits.

**Why did we abstract the database?**
1.  **Testing Strategy**: We can execute extreme unit tests across our entire financial logic plane by faking the `dyn AccountRepository` without needing a massive seeded PostgreSQL container to run `cargo test`.
2.  **Future Proofing**: If the CEO decides tomorrow to run our Audit Logs to ClickHouse instead of Postgres, we simply build a `ClickHouseAuditRepository` in the Infra crate. Zero business logic is touched.

---

## 3. The Double-Entry Finance Engine
**Why is every transaction hyper-strict?**

In our `/api/v1/finance` namespace we process massive volumes. An order from a restaurant inside the RMS or a monthly rent cycle firing in the PMS both trigger down to the `ledger`.

We adhere to a strict double-entry standard `amount NUMERIC(14,2)` requiring balancing. Because we use `rust_decimal`, we avoid the catastrophic IEEE-754 floating-point errors inherent in Node.js floats. By enforcing `amount` and `direction` (Debit / Credit), we guarantee our executive accounting reports track every lost cent back to a specific domain action globally.
