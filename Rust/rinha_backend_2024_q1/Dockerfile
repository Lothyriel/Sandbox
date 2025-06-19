# Build stage
FROM rust:1.76 as builder

COPY src ./src
COPY Cargo.toml ./

RUN cargo build --release

# Prod stage
FROM debian:stable-slim
COPY --from=builder /target/release/rinha_backend_2024_q1 /

EXPOSE 9999

ENTRYPOINT ["./rinha_backend_2024_q1"]
