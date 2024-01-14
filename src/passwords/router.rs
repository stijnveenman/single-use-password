use axum::{
    extract::{Path, Query, State},
    routing::get,
    Json, Router,
};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{
    app_context::AppContext,
    app_result::{AppError, AppResult},
};

pub fn router() -> Router<AppContext> {
    Router::new()
        .route("/", get(get_passwords).post(create_password))
        .route("/:id", get(get_password))
}

#[derive(Serialize, sqlx::FromRow, Debug)]
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
    let key: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let password = Password {
        id: Uuid::new_v4(),
        password: payload.password,
        key,
    };

    let result = sqlx::query!(
        "INSERT INTO passwords (id, key, password) VALUES ($1, $2, $3)",
        password.id,
        password.key,
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

#[derive(Deserialize, Debug)]
struct PasswordKeyParams {
    key: String,
}

async fn get_password(
    State(context): State<AppContext>,
    Path(id): Path<Uuid>,
    params: Query<PasswordKeyParams>,
) -> AppResult<Password> {
    let password: Password = sqlx::query_as!(Password, "SELECT * FROM passwords WHERE id = $1", id)
        .fetch_one(&context.db)
        .await
        .map_err(|e| {
            warn!("Error getting password {}", e);
            AppError::NotFound
        })?;

    if password.key != params.key {
        return Err(AppError::NotFound);
    }

    let result = sqlx::query!("DELETE FROM passwords WHERE id = $1", id)
        .execute(&context.db)
        .await;

    match result {
        Ok(_) => Ok(Json(password)),
        Err(e) => {
            warn!("Error deleting password: {}", e);
            Err(AppError::Random)
        }
    }
}
