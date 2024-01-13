mod app_context;
mod app_result;
mod config;
mod passwords;

use app_result::{AppError, AppResult};
use axum::{routing::get, Router};
use clap::Parser;
use config::Config;
use dotenv::dotenv;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;

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
        .nest("/passwords", passwords::router::new())
        .route("/failing", get(failing))
        .with_state(state);

    tracing::info!("Listening on {}", config.server_url);

    let listener = tokio::net::TcpListener::bind(config.server_url)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn failing() -> AppResult<Value> {
    Err(AppError::Random)
}
