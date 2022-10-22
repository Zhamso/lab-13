use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router};

use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use axum::extract::Path;  
use serde_json::{json};

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/", get(root))
        .route("/v1/todos/:id", get(json_hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn json_hello(Path(id): Path<String>) -> impl IntoResponse {
    let greeting = id.as_str();
    let hello = String::from("Hello ");

    (StatusCode::OK, Json(json!({"id": id.as_str(), "message": "Just do it!",
    "priority": "A",
    "assigned": "user@example.com"})))
}