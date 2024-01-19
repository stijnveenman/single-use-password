use leptos::*;
use leptos_router::use_params_map;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct Password {
    pub id: Uuid,
    pub key: String,
    pub password: String,
}

#[component]
pub fn PasswordPage() -> impl IntoView {
    let params = use_params_map();

    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .and_then(|id| Uuid::parse_str(&id).ok())
            .map(|id| id.to_string())
    };

    view! {
        <h1>"Welcome to Single-Use-Password"</h1>
        <p>"Enter password to unlock:"</p>
        <p>{id}</p>
    }
}
