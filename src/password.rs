use leptos::{ev::SubmitEvent, html::Input, *};
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

    let (name, set_name) = create_signal("Uncontrolled".to_string());
    let input_element: NodeRef<Input> = create_node_ref();
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };

    view! {
        <h1>"Welcome to Single-Use-Password"</h1>
        <p>"Enter password to unlock:"</p>
        <p>{id}</p>
        <form on:submit=on_submit>
            <input type="text" value=name node_ref=input_element/>
            <input type="submit" value="Submit"/>
        </form>
        <p>"Name is: " {name}</p>
    }
}
