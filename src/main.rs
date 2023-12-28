use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::{postgres::PgPoolOptions, types::Uuid, PgPool};
use thiserror::Error;

#[derive(Clone)]
struct AppContext {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:postgres@localhost:5433/single-use-password")
        .await
        .unwrap();

    let state = AppContext { db: pool };

    let app = Router::new()
        .route("/json", get(json))
        .route("/passwords", get(get_passwords))
        .route("/failing", get(failing))
        .with_state(state);

    //let row = sqlx::query!("SELECT * FROM passwords")
    //    .fetch_all(&pool)
    //    .await
    //    .unwrap();

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct Password {
    id: Uuid,
    key: String,
    password: String,
}

async fn get_passwords(State(context): State<AppContext>) -> Json<Vec<Password>> {
    let data = sqlx::query_as!(Password, "SELECT * FROM passwords")
        .fetch_all(&context.db)
        .await
        .unwrap();

    Json(data)
}

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json() -> Json<Value> {
    Json(json!({ "data": 42 }))
}

#[derive(Error, Debug)]
enum AppError {
    #[error("Random intermittent error")]
    Random,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        match self {
            AppError::Random => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error":"Random error"})),
            ),
        }
        .into_response()
    }
}

async fn failing() -> Result<Json<Value>, AppError> {
    Err(AppError::Random)
}
