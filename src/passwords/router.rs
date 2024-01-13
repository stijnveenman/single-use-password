use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{app_context::AppContext, app_result::AppResult};

pub fn router() -> Router<AppContext> {
    Router::new().route("/", get(get_passwords).post(create_password))
}

#[derive(Serialize)]
struct Password {
    id: Uuid,
    key: String,
    password: String,
}

async fn get_passwords(State(context): State<AppContext>) -> AppResult<Vec<Password>> {
    let data = sqlx::query_as!(Password, "SELECT * FROM passwords")
        .fetch_all(&context.db)
        .await
        .unwrap();

    Ok(Json(data))
}

#[derive(Deserialize)]
struct CreatePassword {
    password: String,
}

async fn create_password(
    State(context): State<AppContext>,
    Json(payload): Json<CreatePassword>,
) -> AppResult<Password> {
    Err(crate::app_result::AppError::NotImplemented)
}
