use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::use_params_map;
use uuid::Uuid;

use crate::password::{unlock_password, Password};

#[component]
fn UnlockForm(#[prop(into)] on_unlock: Callback<Password>) -> impl IntoView {
    let params = use_params_map();
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None);

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
                set_error(None);
                set_loading(true);
                let result = unlock_password(id, key).await;
                set_loading(false);
                match result {
                    Ok(password) => on_unlock(password),
                    Err(ServerFnError::ServerError(e)) => set_error(Some(e)),
                    Err(_) => set_error(Some("Unknown error".into())),
                }
            });
        }
    };

    view! {
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img
                class="mx-auto h-10 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                alt="Your Company"
            />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                Unlock password
            </h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6" on:submit=on_submit>
                <input
                    node_ref=input_element
                    type="text"
                    placeholder="Enter key"
                    class="input input-bordered w-full "
                />

                <button class="btn btn-primary w-full" disabled=loading>
                    {move || match loading.get() {
                        true => "Unlocking",
                        false => "Unlock",
                    }}

                    <Show when=loading>
                        <span class="loading loading-spinner"></span>
                    </Show>
                </button>

                <Show when=move || error.get().is_some()>
                    <div role="alert" class="alert alert-error">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="stroke-current shrink-0 h-6 w-6"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            ></path>
                        </svg>
                        <span>{error}</span>
                    </div>
                </Show>
            </form>

            <p class="mt-10 text-center text-sm text-gray-500">
                Need to share a password?
                <a href="#" class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">
                    Create one
                </a>
            </p>
        </div>
    }
}

#[component]
fn PasswordDisplay(password: Password) -> impl IntoView {
    view! {
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img
                class="mx-auto h-10 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                alt="Your Company"
            />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                Password unlocked
            </h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <input
                type="text"
                class="input input-bordered w-full !cursor-default"
                value=password.password
                disabled
            />
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <p class="mt-10 text-center text-sm text-gray-500">
                Need to share a password?
                <a href="#" class="font-semibold leading-6 text-indigo-600 hover:text-indigo-500">
                    Create one
                </a>
            </p>
        </div>
    }
}

#[component]
pub fn PasswordForm() -> impl IntoView {
    let (password, set_password) = create_signal(None);

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            {move || match password.get() {
                None => {
                    view! { <UnlockForm on_unlock=move |password| set_password(Some(password))/> }
                        .into_view()
                }
                Some(password) => view! { <PasswordDisplay password=password/> }.into_view(),
            }}

        </div>
    }
}
