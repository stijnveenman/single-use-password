use axum::{extract::State, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;

use crate::{
    app_context::AppContext,
    app_result::{AppError, AppResult},
};

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
    let password = Password {
        id: Uuid::new_v4(),
        password: payload.password,
        key: "".to_string(),
    };

    let result = sqlx::query!(
        "INSERT INTO passwords (id, key, password) VALUES ($1, $2, $3)",
        password.id,
        "",
        password.password
    )
    .execute(&context.db)
    .await;

    match result {
        Ok(_) => Ok(Json(password)),
        Err(e) => {
            info!("Unexpected db error {}", e);
            Err(AppError::Unexpected)
        }
    }
}
