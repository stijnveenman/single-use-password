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
async fn unlock_password(id: Uuid, key: String) -> Result<Password, ServerFnError> {
    let pool = app_context::pool()?;

    let password = sqlx::query_as!(Password, "SELECT * FROM passwords WHERE id = $1", id)
        .fetch_one(&pool)
        .await
        .map_err(|_| ServerFnError::ServerError("Not found".into()))?;

    if password.key != key {
        return Err(ServerFnError::ServerError("Invalid password".into()));
    }

    let result = sqlx::query!("DELETE FROM passwords WHERE id = $1", id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(password),
        Err(e) => Err(ServerFnError::ServerError(e.to_string())),
    }
}

#[component]
pub fn PasswordPage() -> impl IntoView {
    let params = use_params_map();
    let (password, set_password) = create_signal(None);

    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .and_then(|id| Uuid::parse_str(&id).ok())
    };

    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let key = input_element().expect("<input> to exist").value();

        if let Some(id) = id() {
            spawn_local(async move {
                let result = unlock_password(id, key).await;
                match result {
                    Ok(password) => set_password(Some(password)),
                    Err(e) => logging::log!("{:?}", e),
                }
            });
        }
    };

    view! {
        <h1>"Welcome to Single-Use-Password"</h1>

        {move || match password.get() {
            Some(password) => {
                view! {
                    <div>
                        <p>Password:</p>
                        <h2>{password.password}</h2>
                    </div>
                }
                    .into_any()
            }
            None => {
                view! {
                    <form on:submit=on_submit>
                        <p>"Enter password to unlock:"</p>
                        <input type="text" node_ref=input_element/>
                        <input type="submit" value="Submit"/>
                    </form>
                }
                    .into_any()
            }
        }}
    }
}
