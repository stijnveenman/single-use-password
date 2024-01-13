mod app_context;
mod config;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use clap::Parser;
use config::Config;
use dotenv::dotenv;
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, types::Uuid};
use thiserror::Error;

use crate::app_context::AppContext;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    let state = AppContext { db: pool };

    let app = Router::new()
        .route("/passwords", get(get_passwords))
        .route("/failing", get(failing))
        .with_state(state);

    tracing::info!("Listening on {}", config.server_url);

    let listener = tokio::net::TcpListener::bind(config.server_url)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Password {
    id: Uuid,
    key: String,
    password: String,
}

async fn get_passwords(State(context): State<AppContext>) -> Json<Vec<Password>> {
    let data = sqlx::query_as!(Password, "SELECT * FROM passwords")
        .fetch_all(&context.db)
        .await
        .unwrap();

    Json(data)
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
