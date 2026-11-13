# Schema and invariants

## Core relationships
- `accounts` 1:N `journal_lines`; `journal_entries` 1:N `journal_lines`.
- `budget_pools` 1:N `budget_movements`.
- `budget_periods` N:M `budget_pools` via `budget_targets`.
- `period_snapshots` preserve closed-period values.
- `audit_events` record changes; their generation is not implemented.

## Exercises to implement (not solved)
1. Define journal entry posting as a single database transaction.
2. Enforce equal debit/credit totals, minimum two lines, currency rules, and posted immutability.
3. Decide how to enforce constraints under concurrent requests and direct database writes.
4. Implement transaction dates vs recorded/modified timestamps.
5. Define budget movement signs, paired transfers, rollovers, and reconciliation to eligible cash.
6. Make monthly closing atomic and snapshots immutable.
7. Add idempotency and request validation before mobile synchronization.
8. Decide whether `NUMERIC(20,2)` suffices for all supported currencies.

**Note:** SQL constraints alone do not enforce all these invariants. This is a teaching scaffold, not production-safe financial storage.
