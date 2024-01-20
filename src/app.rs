use crate::{
    create_password_form::CreatePasswordForm,
    error_template::{AppError, ErrorTemplate},
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
            <Html
                attributes=vec![("data-theme", Attribute::String("corporate".into()))]
                class=" h-full bg-white"
            />
            <Body class="h-full"/>
            <main class="h-full">
                <Routes>
                    <Route path="create" view=CreatePasswordForm/>
                    <Route path=":id" view=PasswordForm/>
                </Routes>
            </main>
        </Router>
    }
}
