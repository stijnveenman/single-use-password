mod app_context;
mod app_result;
mod config;
mod passwords;

use app_result::{AppError, AppResult};
use clap::Parser;
use config::Config;
use dotenv::dotenv;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;

use crate::app_context::AppContext;
#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::{routing::get, routing::post, Router};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use single_use_password::app::*;
    use single_use_password::fileserv::file_and_error_handler;

    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    let state = AppContext { db: pool };

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let app = Router::new()
        .nest("/passwords", passwords::router())
        .route("/failing", get(failing))
        .with_state(state)
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns))
        .leptos_routes(&leptos_options, routes, App)
        .fallback(file_and_error_handler)
        .with_state(leptos_options);

    tracing::info!("Listening on {}", config.server_url);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn failing() -> AppResult<Value> {
    Err(AppError::Random)
}
