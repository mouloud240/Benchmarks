use axum::{
    routing::{get, post},
    Router, Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Serialize)]
struct Greeting {
    #[serde(rename = "hello word v1 ")]
    message: &'static str,
}

#[derive(Deserialize, Debug)]
struct CreateGreetingRequest {
    id: i64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(rename = "greetDate")]
    greet_date: String,
}

#[derive(Serialize)]
struct GreetingData {
    id: i64,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(rename = "greetDate")]
    greet_date: String,
}

#[derive(Serialize)]
struct CreateGreetingResponse {
    success: bool,
    data: GreetingData,
}

#[derive(Serialize)]
struct ErrorResponse {
    success: bool,
    error: String,
}

async fn greetings() -> Json<Greeting> {
    Json(Greeting { message: "Hello, World!" })
}

async fn create_greeting(
    Json(payload): Json<CreateGreetingRequest>,
) -> Response {
    // Validate that greet_date is a valid ISO 8601 date
    if chrono::DateTime::parse_from_rfc3339(&payload.greet_date).is_err() {
        let error = ErrorResponse {
            success: false,
            error: format!("Invalid date format for greetDate. Expected ISO 8601 format."),
        };
        return (StatusCode::BAD_REQUEST, Json(error)).into_response();
    }

    let response = CreateGreetingResponse {
        success: true,
        data: GreetingData {
            id: payload.id,
            name: payload.name,
            message: payload.message,
            greet_date: payload.greet_date,
        },
    };

    (StatusCode::OK, Json(response)).into_response()
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

    // Build our application with routes
    let app = Router::new()
        .route("/api/v1/greetings", get(greetings).post(create_greeting));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
