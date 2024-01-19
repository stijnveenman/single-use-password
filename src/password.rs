use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_params_map;
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

    let result = sqlx::query!("DELETE FROM passwords WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(password),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}
