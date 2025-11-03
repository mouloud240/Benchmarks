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
COPY Cargo.toml ./

# Prepare a small dummy crate so Docker can cache dependency compilation
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Generate lockfile (if one isn't present) and build dependencies to cache them
RUN cargo generate-lockfile
RUN cargo build --release

# Copy actual source and build the real binary
COPY src ./src
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
