FROM rust:1.56.0-slim AS builder 
WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR /app

RUN apt-get update -y && apt-get instal -y --no-install-recommends openssl && apt-get autoremove -y && apt-get clean &&rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/zero2prod zero2prod
COPY configuration configuration
ENV APP_ENVIRONMENT production
ENTRYPOINT ["./zero2prod"]