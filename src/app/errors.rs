use axum::{http::StatusCode, Json, response::IntoResponse};
use serde::Serialize;

#[derive(Serialize)]
struct ErrorBody {
    status: String,
    message: String,
}

pub enum AppError {
    NotFound(String),
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        (status, Json(ErrorBody { status: "error".to_string(), message })).into_response()
    }
}
