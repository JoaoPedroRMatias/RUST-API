FROM rust:1.82-slim AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/RustAPI*

COPY src ./src

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/RustAPI .

EXPOSE 3000

CMD ["./RustAPI"]