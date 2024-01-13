use axum::{extract::State, routing::get, Json, Router};
use serde::Serialize;
use uuid::Uuid;

use crate::app_context::AppContext;

pub fn new() -> Router<AppContext> {
    Router::new().route("/", get(get_passwords))
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
