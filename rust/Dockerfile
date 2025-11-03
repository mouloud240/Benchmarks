# ============================
# 🏗️ Builder Stage
# ============================
FROM rust:1.83-slim AS builder

WORKDIR /usr/src/greetings_rust

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config libssl-dev ca-certificates build-essential \
    && rm -rf /var/lib/apt/lists/*

# Copy the full project and build
COPY Cargo.toml Cargo.lock ./
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
