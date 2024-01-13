use axum::{http::StatusCode, response::IntoResponse, Json};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Random intermittent error")]
    Random,
    #[allow(dead_code)]
    #[error("This feature is not implemented yet")]
    NotImplemented,
    #[error("Unexpected error")]
    Unexpected,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            match self {
                AppError::Random => StatusCode::BAD_REQUEST,
                AppError::NotImplemented => StatusCode::BAD_REQUEST,
                AppError::Unexpected => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Json(json!({"error":self.to_string()})),
        )
            .into_response()
    }
}

pub type AppResult<T> = Result<Json<T>, AppError>;
