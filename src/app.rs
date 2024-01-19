use crate::{
    app_context,
    error_template::{AppError, ErrorTemplate},
    password::{Password, PasswordPage},
    password_form::PasswordForm,
};
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Title text="Welcome to Leptos"/>

        <Link rel="shortcut icon" type_="image/ico" href="/static/favicon.ico"/>
        <Stylesheet id="leptos" href="/static/output.css"/>

        // content for this welcome page
        <Router fallback=|| {
            let mut outside_errors = Errors::default();
            outside_errors.insert_with_default_key(AppError::NotFound);
            view! { <ErrorTemplate outside_errors/> }.into_view()
        }>
            <Html class="h-full bg-white"/>
            <Body class="h-full"/>
            <main class="h-full">
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/new" view=PasswordForm/>
                    <Route path=":id" view=PasswordPage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(GetPassword, "/api")]
pub async fn get_password() -> Result<Vec<Password>, ServerFnError> {
    tracing::info!("password");
    let pool = app_context::pool()?;
    let data = sqlx::query_as!(Password, "SELECT * FROM passwords")
        .fetch_all(&pool)
        .await
        .unwrap();

    tracing::info!("{:?}", data);

    Ok(data)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let (count, set_count) = create_signal(0);
    let on_click = move |_| set_count.update(|count| *count += 1);

    let passwords = create_resource(|| (), move |_| get_password());

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <Suspense fallback=|| ()>
            <h2>
                {move || { passwords.get().map(|data| data.unwrap().len().to_string()) }}
                " passwords in store"
            </h2>
        </Suspense>
        <button on:click=on_click>"Click Me: " {count}</button>
    }
}
