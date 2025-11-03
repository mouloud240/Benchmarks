# greetings_rust

Small Actix-web server returning the same JSON payload used by the FastAPI example.

Endpoint:

- GET /api/v1/greetings  -> {"hello word v1 ": "Hello, World!"}

Run locally:

1. Install Rust toolchain (rustup/cargo).
2. From this directory run:

    cargo run --release

The server binds to 127.0.0.1:8001 by default.

Docker
------

This repository includes a multi-stage `Dockerfile` that builds the Rust binary in a builder image and copies the release binary into a slim runtime image.

Build the image from the `rust/` directory:

    docker build -t greetings_rust:latest .

Run the container and map the port:

    docker run --rm -p 8001:8001 greetings_rust:latest

The container will serve the same endpoint at `http://127.0.0.1:8001/api/v1/greetings`.

Notes:

- The builder stage uses `rust:1.81`. If you want a smaller final image, consider using a static build (musl) or a distroless base.
- If building on CI, ensure Docker has enough memory and network access to fetch crates.
