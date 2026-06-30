use axum::Json;
use serde::Serialize;
use crate::app::errors::AppError;
use crate::app::state::AppState;

#[derive(Serialize)]
pub struct MessageResponse {
    status: String,
    message: String,
}

pub async fn hello_handler(state: AppState) -> Result<Json<MessageResponse>, AppError> {
    Ok(Json(MessageResponse {
        status: "success".to_string(),
        message: format!("Hello from {}!", state.app_name),
    }))
}
