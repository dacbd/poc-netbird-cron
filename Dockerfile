ARG RUST_VERSION=1.82.0

FROM rust:${RUST_VERSION}-slim-bookworm as builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config
WORKDIR /app
COPY . .
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && \
    cp ./target/release/poc-netbird-cron /poc-netbird-cron

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates
COPY --from=builder /poc-netbird-cron /usr/local/bin/poc-netbird-cron
ENTRYPOINT [ "poc-netbird-cron" ]
