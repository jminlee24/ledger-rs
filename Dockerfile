FROM rust:1-bookworm AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p ledger-api
FROM debian:bookworm-slim
RUN useradd --system --uid 10001 ledger
COPY --from=builder /app/target/release/ledger-api /usr/local/bin/ledger-api
USER ledger
EXPOSE 8000
CMD ["ledger-api"]
