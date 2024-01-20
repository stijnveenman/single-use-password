use leptos::*;
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::app_context;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Password {
    pub id: Uuid,
    pub key: String,
    pub password: String,
}

#[allow(unreachable_code)]
#[server(UnlockPassword)]
pub async fn unlock_password(id: Uuid, key: String) -> Result<Password, ServerFnError> {
    let pool = app_context::pool()?;

    let password = sqlx::query_as!(Password, "SELECT * FROM passwords WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| ServerFnError::ServerError("Not found".into()))?;

    if password.key != key {
        return Err(ServerFnError::ServerError("Invalid key".into()));
    }

    #[cfg(debug_assertions)]
    return Ok(password);

    let result = sqlx::query!("DELETE FROM passwords WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(password),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[server(CreatePassword)]
pub async fn create_password(password: String) -> Result<Password, ServerFnError> {
    let pool = app_context::pool()?;

    let key: String = OsRng
        .sample_iter(&Alphanumeric)
        .take(10)
        .map(char::from)
        .collect();

    let password = Password {
        id: Uuid::new_v4(),
        password,
        key,
    };

    let result = sqlx::query!(
        "INSERT INTO passwords (id, key, password) VALUES ($1, $2, $3)",
        password.id,
        password.key,
        password.password
    )
    .execute(&pool)
    .await;

    match result {
        Ok(_) => Ok(password),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
