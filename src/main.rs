use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::{json, Value};
use thiserror::Error;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/json", get(json))
        .route("/failing", get(failing));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Random intermittent error")]
    Random,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Random => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Random error"})),
            ),
        }
        .into_response()
    }
}

async fn failing() -> Result<Json<Value>, AppError> {
    Err(AppError::Random)
}
