use axum::{routing::get, Router, Json};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Greeting {
    #[serde(rename = "hello word v1 ")]
    message: &'static str,
}

async fn greetings() -> Json<Greeting> {
    Json(Greeting { message: "Hello, World!" })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Initialize tracing subscriber for logs (reads RUST_LOG if set)
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info"))
        )
        .init();

    // Build our application with a single route: GET /api/v1/greetings
    let app = Router::new().route("/api/v1/greetings", get(greetings));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
