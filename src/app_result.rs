use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
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

pub type AppResult<T> = Result<Json<T>, AppError>;
