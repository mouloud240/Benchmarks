use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use chrono::NaiveDate;


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

#[derive(Deserialize, Serialize)]
struct ValidationSchema {
    id: i32,
    name: String,
    message: Option<String>,
    greetDate: NaiveDate,
}

async fn validate_data(Json(data): Json<ValidationSchema>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "success",
        "data": data,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let app = Router::new()
        .route("/api/v1/greetings", get(greetings))
        .route("/api/v1/greetings", post(validate_data));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8001));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Server is running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
