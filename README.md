# Ledger RS — learning scaffold

Rust workspace for a self-hosted personal accounting and envelope-budgeting app. **No accounting engine is implemented.** The only working API route is `/health`. Database tables are a starting schema, not a complete set of accounting safeguards.

## Requirements
Rust stable, Docker + Compose, GNU Make, and SQLx CLI:

```sh
cargo install sqlx-cli --no-default-features --features postgres,rustls
cp .env.example .env
# Edit .env: change password in BOTH POSTGRES_PASSWORD and DATABASE_URL
make db-up
make migrate
make check
make test
make run
curl http://localhost:8000/health
```

`sqlx migrate revert` requires a corresponding `.down.sql` migration (not provided). To change schema during development, add a new migration using `sqlx migrate add <name>`.

## Workspace
- `domain`: domain types and rules (your implementation)
- `application`: use cases and transaction orchestration (your implementation)
- `persistence`: SQLx adapters (your implementation)
- `api`: Axum transport (only health route implemented)
- `migrations`: initial PostgreSQL schema
- `docs`: design exercises and endpoint plan

## Suggested build order
1. Define domain types and error types.
2. Implement posting rules with tests before database writes.
3. Add SQLx repository and atomic posting transaction.
4. Implement budgets and reconciliation.
5. Add endpoints, auth, and audit trail.
6. Add desktop/PWA clients and sync.

## Security
The DB port is bound to localhost. The API development listener is `0.0.0.0:8000`, so **do not run it on an untrusted network**; restrict it with your firewall or change the listener to `127.0.0.1`. Do not deploy the provided Dockerfile as a publicly accessible service before adding authentication and TLS. Back up PostgreSQL before storing real financial data.
