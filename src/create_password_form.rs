use leptos::{ev::SubmitEvent, html::Input, *};

use crate::password::{create_password, Password};

#[component]
fn CreatePassword(#[prop(into)] on_create: Callback<Password>) -> impl IntoView {
    let (loading, set_loading) = create_signal(false);
    let (error, set_error) = create_signal(None);

    let input_element: NodeRef<Input> = create_node_ref();

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let password = input_element().expect("<input> to exist").value();

        spawn_local(async move {
            set_error(None);
            set_loading(true);
            let result = create_password(password).await;
            set_loading(false);
            match result {
                Ok(password) => on_create(password),
                Err(ServerFnError::ServerError(e)) => set_error(Some(e)),
                Err(_) => set_error(Some("Unknown error".into())),
            }
        });
    };

    view! {
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img
                class="mx-auto h-10 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                alt="Your Company"
            />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                Share password
            </h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <form class="space-y-6" on:submit=on_submit>
                <input
                    node_ref=input_element
                    type="text"
                    name="password"
                    placeholder="Enter password"
                    class="input input-bordered w-full "
                />

                <button class="btn btn-primary w-full" disabled=loading>
                    {move || match loading.get() {
                        true => "Locking",
                        false => "Lock",
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
        </div>
    }
}

#[component]
fn ShowPassword(password: Password) -> impl IntoView {
    view! {
        <div class="sm:mx-auto sm:w-full sm:max-w-sm">
            <img
                class="mx-auto h-10 w-auto"
                src="https://tailwindui.com/img/logos/mark.svg?color=indigo&shade=600"
                alt="Your Company"
            />
            <h2 class="mt-10 text-center text-2xl font-bold leading-9 tracking-tight text-gray-900">
                Password created
            </h2>
        </div>

        <div class="mt-10 sm:mx-auto sm:w-full sm:max-w-sm">
            <label class="form-control w-full">
                <div class="label">
                    <span class="label-text font-bold">Key:</span>
                </div>
                <div class="join w-full">
                    <input
                        type="text"
                        name="key"
                        class="input input-bordered w-full !cursor-default"
                        value=&password.key
                        disabled
                    />

                    <button
                        class="btn"
                        onclick=format!(
                            "navigator.clipboard.writeText('{}')",
                            &password.key.replace('\'', "\\'"),
                        )
                    >

                        Copy
                    </button>
                </div>
            </label>

        </div>
    }
}

#[component]
pub fn CreatePasswordForm() -> impl IntoView {
    let (password, set_password) = create_signal(None);

    view! {
        <div class="flex min-h-full flex-col justify-center px-6 py-12 lg:px-8">
            {move || match password.get() {
                None => {
                    view! {
                        <CreatePassword on_create=move |password| set_password(Some(password))/>
                    }
                        .into_view()
                }
                Some(password) => view! { <ShowPassword password=password/> }.into_view(),
            }}

        </div>
    }
}
