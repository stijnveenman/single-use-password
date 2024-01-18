use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::PgPool;

#[derive(Clone, FromRef)]
pub struct AppContext {
    pub leptos_options: LeptosOptions,
    pub db: PgPool,
}
