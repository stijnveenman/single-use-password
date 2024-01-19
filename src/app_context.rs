use cfg_if::cfg_if;

cfg_if! {
if #[cfg(feature = "ssr")]  {

use axum::extract::FromRef;
use leptos::{use_context, LeptosOptions, ServerFnError};
use sqlx::PgPool;


#[derive(Clone, FromRef)]
pub struct AppContext {
    pub leptos_options: LeptosOptions,
    pub db: PgPool,
}

#[allow(dead_code)]
pub fn pool() -> Result<PgPool, ServerFnError> {
    use_context::<PgPool>().ok_or_else(|| ServerFnError::ServerError("Pool missing.".into()))
}


}}
