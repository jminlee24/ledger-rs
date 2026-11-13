.PHONY: help db-up db-down migrate migrate-revert run fmt lint test check
help:
	@echo 'make db-up | migrate | run | fmt | lint | test | check'
db-up:
	docker compose up -d db
db-down:
	docker compose down
migrate:
	set -a; . ./.env; set +a; sqlx migrate run
migrate-revert:
	set -a; . ./.env; set +a; sqlx migrate revert
run:
	cargo run -p ledger-api
fmt:
	cargo fmt --all
lint:
	cargo clippy --workspace --all-targets -- -D warnings
test:
	cargo test --workspace
check:
	cargo check --workspace
