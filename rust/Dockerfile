# ============================
# 🏗️ Builder Stage
# ============================
FROM rust:1.83-slim AS builder

WORKDIR /usr/src/greetings_rust

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests for caching
COPY Cargo.toml Cargo.lock ./

# Build deps first
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf target/release/deps/greetings_rust*

# Copy actual source
COPY src ./src

# Build binary
RUN cargo build --release

# ============================
# 🚀 Runtime Stage
# ============================
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/greetings_rust/target/release/greetings_rust /usr/local/bin/greetings_rust

EXPOSE 8001
ENV RUST_LOG=info

CMD ["/usr/local/bin/greetings_rust"]
