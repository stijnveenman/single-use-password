use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, types::Uuid};
use thiserror::Error;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/json", get(json))
        .route("/failing", get(failing));

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5433/single-use-password")
        .await
        .unwrap();

    // Make a simple query to return the given parameter (use a question mark `?` instead of `$1` for MySQL)
    let row: Vec<(Uuid, String, String)> = sqlx::query_as("SELECT * FROM passwords")
        .bind(150_i64)
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:?}", row);

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
