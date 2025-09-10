FROM rust:1.89.0 AS builder
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM ubuntu:latest
WORKDIR /app
COPY --from=builder /app/target/release/rus3 /usr/local/bin/rus3

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

ENV RUST_LOG=info

EXPOSE 3000

CMD ["rus3"]
