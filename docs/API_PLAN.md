# API contract (design prompts, no implementations)

Proposed endpoints:
- GET /health (implemented)
- GET /accounts; POST /accounts
- GET /journal-entries; POST /journal-entries (draft); POST /journal-entries/{id}/post
- POST /journal-entries/{id}/reverse
- GET /budget-pools; POST /budget-pools
- GET /budget-periods; POST /budget-periods; POST /budget-periods/{id}/close
- POST /budget-movements; GET /budget-movements
- GET /reports/trial-balance; GET /reports/budget?period=...

Before exposing any non-health endpoint remotely, add authentication, authorization, request limits, and TLS/private network access.
