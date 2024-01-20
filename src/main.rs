mod app_result;
mod config;
mod passwords;

use cfg_if::cfg_if;

use app_result::{AppError, AppResult};
use clap::Parser;
use config::Config;
use dotenv::dotenv;
use serde_json::Value;
use sqlx::postgres::PgPoolOptions;

cfg_if! {
if #[cfg(feature = "ssr")]  {
    mod app_context;

    use crate::app_context::AppContext;
    use axum::{routing::get, Router, extract::{State, RawQuery, Path}, body::Body as AxumBody, http::{Request, header::HeaderMap}, response::{Response, IntoResponse}};
    use leptos::*;
    use leptos_axum::{generate_route_list, LeptosRoutes, handle_server_fns_with_context};
    use single_use_password::app::*;
    use single_use_password::fileserv::file_and_error_handler;


    async fn server_fn_handler(State(app_state): State<AppContext>,  path: Path<String>, headers: HeaderMap, raw_query: RawQuery, request: Request<AxumBody>) -> impl IntoResponse {
        tracing::info!("{:?}", path);

        handle_server_fns_with_context(path, headers, raw_query, move || {
            provide_context(app_state.db.clone());
        }, request).await
    }

    async fn leptos_routes_handler(State(app_state): State<AppContext>, req: Request<AxumBody>) -> Response{
        let handler = leptos_axum::render_app_to_stream_with_context(app_state.leptos_options.clone(),
            move || {
                provide_context( app_state.db.clone());
            },
            || view! { <App/> }
        );
        handler(req).await.into_response()
    }

#[tokio::main]
async fn main() {

    dotenv().ok();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = Config::parse();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("should migrate database on start");

    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppContext {
        db: pool,
        leptos_options,
    };

    let app = Router::new()
        .nest("/passwords", passwords::router())
        .route("/failing", get(failing))
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        .leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .fallback(file_and_error_handler)
        .with_state(state);

    tracing::info!("Listening on {}", config.server_url);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
     }}

async fn failing() -> AppResult<Value> {
    Err(AppError::Random)
}
